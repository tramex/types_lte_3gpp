#!/bin/env python3
import subprocess
import requests
from bs4 import BeautifulSoup
import tempfile
import re
import zipfile
from os import mkdir, listdir
from os.path import exists, join, isfile, basename
from docx_asn1 import extract_text_from_docx
from sys import argv


def download_one_spec(serie):
    params = {"sortby": "daterev"}
    url = f"https://www.3gpp.org/ftp/Specs/archive/36_series/{serie}"
    file = requests.get(
        url,
        params=params,
    )
    soup = BeautifulSoup(file.content, "html.parser")
    table = soup.find("table")
    tbody = table.find("tbody")
    link = tbody.find("a")
    path_to_zip = tempfile.gettempdir() + "/3gpp_build"
    if not exists(path_to_zip):
        mkdir(path_to_zip)
    path_to_zip_file = path_to_zip + "/3gpp.zip"
    with open(path_to_zip_file, "wb") as f:
        f.write(requests.get(link["href"]).content)
    out = path_to_zip + f"/output_{serie}"
    with zipfile.ZipFile(path_to_zip_file, "r") as zip_ref:
        zip_ref.extractall(out)
    files = [join(out, f) for f in listdir(out) if isfile(join(out, f))]
    print("Downloaded", files[0])
    return files[0]


def sa1p_patch(text):
    new_text = text.replace("id-CSGMembershipStatus", "id-CSGStatus")
    new_text = text.replace("CSGMembershipStatus", "CSGStatus")
    return new_text


def x2ap_patch(text):
    new_text = text.replace("id-CSGMembershipStatus", "id-CSGStatus")
    new_text = text.replace("CSGMembershipStatus", "CSGStatus")
    return new_text


def rrc_patch(text):
    new_text = re.sub(r".*simultaneousAckNackAndCQI-Format4-Format5-r13.*", "", text)
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
        "patch": sa1p_patch,
    },
    "36.423": {
        "spec": "2452",
        "desc": "X2AP",
        "patch": x2ap_patch,
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


def write_asn1(path, asn1):
    code_asn1 = "asn"
    if not exists(code_asn1):
        mkdir(code_asn1)
    if asn1 is None:
        print(f"Error {one_key}")
    asn_path = code_asn1 + "/" + path
    with open(asn_path, "w") as f:
        f.write(asn1)
    if asn1 == "":
        print(f"Empty ASN1 {one_key} ({spec_ids[one_key]['desc']})")
    print("ASN1", path, "written")
    return asn_path


def translate_to_code(asn_path, code):
    code_path = "src"
    if not exists(code_path):
        mkdir(code_path)
    print("Translating", asn_path, "to", code, end="...")
    try:
        path_code = code_path + "/" + code
        cmd = f"hampi-rs-asn1c  --codec  uper --derive serialize --derive deserialize --module {path_code} -- {asn_path} > /tmp/gpp"
        with open(f"/tmp/gpp_{code}", "w") as outfile:
            subprocess.run(cmd, stdout=outfile, stderr=outfile, shell=True)
        if exists(path_code):
            print("done")
        else:
            raise Exception("Error file not created")
    except Exception as _e:
        print(
            f"Error asn1 translation {one_key} ({spec_ids[one_key]['desc']}) see more at /tmp/gpp_{code}"
        )


should_download = "--download" in argv and len(argv) == 2 or len(argv) == 1
should_compile = "--compile" in argv and len(argv) == 2 or len(argv) == 1

for one_key in spec_ids.keys():
    desc = spec_ids[one_key]["desc"]
    path_asn = f"{one_key}_spec_{desc}.asn"
    if should_download:
        docx = download_one_spec(one_key)
        path = basename(docx).split(".")[0]
        if "start" in spec_ids[one_key]:
            asn1 = extract_text_from_docx(
                docx,
                spec_ids[one_key]["start"],
                spec_ids[one_key]["end"],
                spec_ids[one_key]["add"],
            )
        else:
            asn1 = extract_text_from_docx(docx)
        if "patch" in spec_ids[one_key]:
            asn1 = spec_ids[one_key]["patch"](asn1)
        if asn1 is None or asn1 == "":
            print(f"Error no asn1 {one_key} ({desc})")
            continue
        asn1 = asn1.replace("\U000000a0", " ")
        write_asn1(path_asn, asn1)
    if should_compile:
        path_asn = "asn/" + path_asn
        translate_to_code(path_asn, f"spec_{desc.lower()}.rs")


file = "lib.rs"

list_code = [one_path for one_path in listdir("src") if one_path != file]
with open("src/" + file, "w") as f:
    for one_code in list_code:
        name = one_code.split(".")[0]
        f.write(f"""pub mod {name} {{
    #![allow(warnings)]
    include!("./{name}.rs");
}}\n""")
