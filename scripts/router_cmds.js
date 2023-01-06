#!/usr/bin/env node
// Parses the Abigen'd contract `UniversalRouterCommands`' structs for the `cmds` UniversalRouter
// macro_rules in the form:
//
// StructName -> pub fn struct_name(
//     $($arg),+
// );

const fs = require("fs");
const path = require("path");

const FILE = path.resolve(__dirname, "../src/contracts/bindings/i_universal_router_commands.rs");
const file = fs.readFileSync(FILE, "utf-8");

const order = [
    "V3SwapExactIn",
    "V3SwapExactOut",
    "Permit2TransferFrom",
    "Permit2PermitBatch",
    "Sweep",
    "Transfer",
    "PayPortion",
    // 0x07

    // 0x08..0x10
    "V2SwapExactIn",
    "V2SwapExactOut",
    "Permit2Permit",
    "WrapEth",
    "UnwrapEth",
    "Permit2TransferFromBatch",
    // 0x0e
    // 0x0f

    // 0x10..0x18
    "Seaport",
    "LooksRare721",
    "Nftx",
    "CryptoPunks",
    "LooksRare1155",
    "OwnerCheck721",
    "OwnerCheck1155",
    "SweepErc721",

    // 0x18..0x20
    "X2Y2721",
    "SudoSwap",
    "Nft20",
    "X2Y21155",
    "Foundation",
    "SweepErc1155",
    // 0x1e
    // 0x1f
];

const startStructR = /^pub struct (\w+) \{$/;
const endStructR = /^\}$/;

const snakeCase = string => {
    return string
        .replace(/\W+/g, " ")
        .split(/ |\B(?=[A-Z])/)
        .map(word => word.toLowerCase())
        .join("_");
};

// parse
let inStruct = false;
// name => [fnName, ...args]
const structs = {};
// [key, value]
let curr = [];
for (let line of file.split("\n")) {
    line = line.trim();
    if (inStruct) {
        if (endStructR.test(line)) {
            // struct definition is done
            // add only if it's a "call"
            if (curr[0].endsWith("Call")) {
                structs[curr[0].replace("Call", "")] = [...curr[1]];
            }
            curr = [];
            inStruct = false;
        } else {
            // still in struct definition, push field
            curr[1].push(line.trim().replace("pub ", "").replace(",", ""));
        }
    } else if (startStructR.test(line)) {
        // struct definition started, store name
        inStruct = true;
        line.match(startStructR);
        let res = startStructR.exec(line).pop();
        curr.push(res, [snakeCase(res.replace("Call", ""))]);
    }
}

// "sort"
let _structs = [];
for (const x of order) {
    _structs.push([x, ...structs[x]]);
}

// join and print
for (const [structName, fnName, ...args] of _structs) {
    let _args = args
        .join(",\n    ")
        .replaceAll("ethers_core::types::", "")
        .replaceAll("::std::vec::Vec", "Vec");
    const exp = `\
${structName} => pub fn ${fnName}(
    ${_args}
);\
`;
    console.log(exp);
}
