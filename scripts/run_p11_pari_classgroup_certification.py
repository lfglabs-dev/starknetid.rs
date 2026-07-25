#!/usr/bin/env python3
"""Attempt unconditional PARI certification of the five p=11 degree-36 fields.

Freitas--Naskrecki--Stoll reduce the GRH dependency in their solution of
x^2+y^3=z^11 to class-group computations for L_E=K_E(theta), where K_E is
one of the five degree-12 fields in their Table 5 and theta satisfies the
2-division polynomial of X_0(11).

This driver builds an absolute compositum polynomial, computes a full BNF,
saves it, and runs bnfcertify.  A final FULL_CERTIFICATE=1 is an unconditional
PARI proof of the class group and unit data for that field.  Any failure or
unfinished request has no mathematical interpretation.
"""
from __future__ import annotations

import argparse
import json
import pathlib
import subprocess
import sys

FIELDS = {
    "54a1": (
        "x^12-6*x^10+6*x^9-6*x^8-126*x^7+104*x^6+468*x^5"
        "+258*x^4-456*x^3-1062*x^2-774*x-380"
    ),
    "96a1": (
        "x^12-4*x^11-264*x^7+66*x^6-132*x^5-2112*x^4"
        "-1320*x^3-660*x^2-6240*x-8007"
    ),
    "864a1": (
        "x^12-6*x^11+110*x^9-132*x^8-528*x^7+1100*x^6"
        "+330*x^5-2508*x^4+2134*x^3-594*x^2+456*x-371"
    ),
    "864b1": (
        "x^12-6*x^11+22*x^9+99*x^8-396*x^7+440*x^6"
        "-132*x^5-6501*x^4+33506*x^3-23760*x^2-92418*x+193081"
    ),
    "864c1": (
        "x^12-44*x^9-264*x^8-264*x^7-2266*x^6-4488*x^5"
        "-264*x^4-17644*x^3-7128*x^2+144*x-15191"
    ),
}

THETA_POLYNOMIAL = "x^3-4*x^2-160*x-1264"


def gp_program(label: str) -> str:
    polynomial = FIELDS[label]
    return f'''\
default(parisizemax, 14000000000);
default(nbthreads, 4);
default(realprecision, 96);
default(debug, 1);
F={polynomial};
G={THETA_POLYNOMIAL};
print("FIELD_LABEL={label}");
print("BASE_DEGREE=",poldegree(F));
print("THETA_DEGREE=",poldegree(G));
C=polcompositum(F,G);
print("COMPOSITUM_COMPONENTS=",#C);
if(#C!=1,error("compositum is not a single field"));
P=polredbest(C[1]);
print("ABSOLUTE_DEGREE=",poldegree(P));
print("ABSOLUTE_POLYNOMIAL=",P);
print("FIELD_DISCRIMINANT_START");
NF=nfinit(P);
print("FIELD_DISCRIMINANT=",NF.disc);
print("BNFINIT_START");
B=bnfinit(NF,1);
print("BNFINIT_DONE");
print("CLASS_NUMBER=",B.no);
print("CLASS_GROUP_CYCLIC_FACTORS=",B.cyc);
writebin("p11-{label}-bnf.bin",B);
print("QUOTIENT_CERTIFICATE_START");
CQ=bnfcertify(B,1);
print("QUOTIENT_CERTIFICATE=",CQ);
print("FULL_CERTIFICATE_START");
CF=bnfcertify(B);
print("FULL_CERTIFICATE=",CF);
quit;
'''


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--field", choices=sorted(FIELDS), required=True)
    parser.add_argument("--gp", default="gp")
    parser.add_argument("--program-output", type=pathlib.Path)
    args = parser.parse_args()

    program = gp_program(args.field)
    if args.program_output is not None:
        args.program_output.write_text(program, encoding="utf-8")

    process = subprocess.run(
        [args.gp, "-q"],
        input=program,
        text=True,
        stdout=sys.stdout,
        stderr=sys.stderr,
        check=False,
    )
    return process.returncode


if __name__ == "__main__":
    raise SystemExit(main())
