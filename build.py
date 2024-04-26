import requests
from bs4 import BeautifulSoup
import tempfile
import zipfile
from os import mkdir, listdir, system
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
    return files[0]


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
    },
    "36.413": {
        "spec": "2441",
        "desc": "S1AP",
    },
    "36.423": {
        "spec": "2452",
        "desc": "X2AP",
    },
    "36.443": {
        "spec": "2458",
        "desc": "M2AP",
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
    else:
        print(f"Extracted ASN1 {one_key} ({spec_ids[one_key]['desc']})")
    return asn_path


def translate_to_code(asn_path, code):
    code_path = "src"
    if not exists(code_path):
        mkdir(code_path)
    try:
        code = code_path + "/" + code
        system(
            f"hampi-rs-asn1c  --codec  uper --derive serialize --derive deserialize --module {code} -- {asn_path}"
        )
    except Exception as _e:
        print(f"Error {one_key} ({spec_ids[one_key]['desc']})")


for one_key in spec_ids.keys():
    path_asn = f"{one_key}_spec_{spec_ids[one_key]['desc']}.asn"
    if "--compile" not in argv:
        docx = download_one_spec(one_key)
        path = basename(docx).split(".")[0]
        asn1 = extract_text_from_docx(docx)
        if asn1 is None or asn1 == "":
            print(f"Error {one_key}")
            continue
        write_asn1(path_asn, asn1)
    path_asn = "asn/" + path_asn
    translate_to_code(path_asn, f"spec_{spec_ids[one_key]['desc'].lower()}.rs")


list_code = listdir("src")
with open("src/lib.rs", "w") as f:
    for one_code in list_code:
        f.write(f"mod {one_code.split('.')[0]};\n")
