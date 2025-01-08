#!/bin/env python3

import subprocess
from sys import argv
import tempfile
import re
from pathlib import Path
import zipfile
from os import mkdir, listdir, makedirs
from os.path import exists, join, isfile
from docx_asn1 import extract_text_from_docx
import requests
from bs4 import BeautifulSoup


def download_one_spec(serie):
    """download one spec"""
    params = {"sortby": "daterev"}
    url = f"https://www.3gpp.org/ftp/Specs/archive/36_series/{serie}"
    file = requests.get(url, params=params, timeout=5)
    soup = BeautifulSoup(file.content, "html.parser")
    table = soup.find("table")
    tbody = table.find("tbody")
    link = tbody.find("a")
    path_to_zip = tempfile.gettempdir() + "/3gpp_build"
    if not exists(path_to_zip):
        mkdir(path_to_zip)
    path_to_zip_file = path_to_zip + "/3gpp.zip"
    with open(path_to_zip_file, "wb") as f:
        f.write(requests.get(link["href"], timeout=5).content)
    out = path_to_zip + f"/output_{serie}"
    with zipfile.ZipFile(path_to_zip_file, "r") as zip_ref:
        zip_ref.extractall(out)
    files = [join(out, f) for f in listdir(out) if isfile(join(out, f))]
    print("Downloaded", files[0])
    return files[0]


def rrc_patch(text):
    new_text = re.sub(r".*simultaneousAckNackAndCQI-Format4-Format5-r13.*", "", text)
    new_text = new_text.replace(
        """\t[[condReconfigurationTriggerNR-r17	CondReconfigurationTriggerNR-r17	OPTIONAL-- Need ON
\t]]""",
        "",
    )
    return new_text


spec_ids = {
    "36.321": {
        "spec": "2437",
        "desc": "MAC",
    },
    "36.322": {
        "spec": "2438",
        "desc": "RLC",
    },
    "36.323": {
        "spec": "2439",
        "desc": "PDCP",
    },
    "36.331": {
        "spec": "2440",
        "desc": "RRC",
        "patch": rrc_patch,
    },
    "36.413": {
        "spec": "2441",
        "desc": "S1AP",
        "start": "-- ***************",
        "end": "END",
        "add": True,
    },
    "36.423": {
        "spec": "2452",
        "desc": "X2AP",
    },
    "36.443": {
        "spec": "2458",
        "desc": "M2AP",
        "start": "-- ***************",
        "end": "END",
        "add": True,
    },
    "36.455": {
        "spec": "2462",
        "desc": "LPPa",
    },
}


def write_asn1(one_key, path, asn1):
    """write asn1 to file"""
    code_asn1 = "asn"
    if not exists(code_asn1):
        mkdir(code_asn1)
    if asn1 is None:
        print(f"Error {one_key}")
    asn_path = code_asn1 + "/" + path
    with open(asn_path, "w", encoding="utf-8") as f:
        f.write(asn1)
    if asn1 == "":
        print(f"Empty ASN1 {one_key} ({spec_ids[one_key]['desc']})")
    print("ASN1", path, "written")
    return asn_path


def translate_to_code(one_key, codec, asn_path, code):
    """translate asn1 to rust code"""
    path_code = f"src/{codec}/spec_{code}.rs"
    print("Translating", asn_path, "to", path_code, end="...")
    if not exists(path_code):
        makedirs(Path(path_code).parent, exist_ok=True)
    try:
        cmd = f"hampi-rs-asn1c  --codec {codec} --derive serialize --derive deserialize --module {path_code} -- {asn_path} > /tmp/gpp"
        with open(f"/tmp/gpp_{code}", "w", encoding="utf-8") as outfile:
            subprocess.run(cmd, stdout=outfile, stderr=outfile, shell=True, check=True)
        if exists(path_code):
            print("done")
            return True
        else:
            raise Exception("Error file not created")
    except Exception as e:
        print(
            f"Error asn1 translation ({codec}) {one_key} ({spec_ids[one_key]['desc']}) see more at /tmp/gpp_{code}",
            e,
        )
    return False


def main():
    """main func"""
    should_download = "--download" in argv and len(argv) == 2 or len(argv) == 1
    should_compile = "--compile" in argv and len(argv) == 2 or len(argv) == 1

    codecs = ["uper"]
    # clean
    if "--clean" in argv:
        for one_codec in codecs:
            out_path = f"src/{one_codec}/"
            if exists(out_path):
                for f in listdir(out_path):
                    Path(join(out_path, f)).unlink()
    codecs_text = ""
    for one_codec in codecs:
        for one_key, one_value in spec_ids.items():
            desc = one_value["desc"]
            path_asn = f"{one_key}_spec_{desc}.asn"
            if should_download:
                docx = download_one_spec(one_key)
                if "start" in one_value:
                    asn1 = extract_text_from_docx(
                        docx,
                        one_value["start"],
                        one_value["end"],
                        one_value["add"],
                    )
                else:
                    asn1 = extract_text_from_docx(docx)
                if "patch" in one_value:
                    print(f"Patching {one_key} ({desc})")
                    asn1 = one_value["patch"](asn1)
                if asn1 is None or asn1 == "":
                    print(f"Error no asn1 {one_key} ({desc})")
                    continue
                asn1 = asn1.replace("\U000000a0", " ")
                write_asn1(one_key, path_asn, asn1)
            if should_compile:
                path_asn = "asn/" + path_asn
                for one_codec in codecs:
                    if translate_to_code(one_key, one_codec, path_asn, desc.lower()):
                        codecs_text += f"""#[cfg(feature = "{desc.lower()}")]\n"""
                        codecs_text += f"pub mod spec_{desc.lower()} {{\n"
                        codecs_text += "    #![allow(warnings)]\n"
                        codecs_text += (
                            f"""    include!("./spec_{desc.lower()}.rs");\n"""
                        )
                        codecs_text += "}\n"
        # write mod.rs
        path_mod = f"src/{one_codec}/"
        if not exists(path_mod):
            mkdir(path_mod)
        with open(f"{path_mod}mod.rs", "w", encoding="utf-8") as file:
            file.write(codecs_text)

    file = "lib.rs"

    with open("src/" + file, "w", encoding="utf-8") as f:
        for one_code in codecs:
            f.write(f"pub mod {one_code};\n")


if __name__ == "__main__":
    main()
