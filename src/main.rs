mod lib;
mod extractor;
mod dynamic_rules;
mod static_rules;
// mod lp;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::fs::{File};
use std::io::{BufWriter};
use std::fmt::Display;
use egraph_serialize::EGraph as SerializedEGraph;
use indexmap::IndexMap;
use ordered_float::NotNan;
use egraph_mapping::{SimpleLanguage, convert_to_simple_language_enum, get_node_type};
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;
use anyhow::Context;
use egg::*;

use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str,Value};

pub type Cost = NotNan<f64>;
pub const INFINITY: Cost = unsafe { NotNan::new_unchecked(std::f64::INFINITY) };

pub fn save_egraph_to_json(egraph: &EGraph<SimpleLanguage, ()>, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let json_rep = serde_json::to_string_pretty(&egraph).unwrap();
    fs::write(&file_path, json_rep)?;
    Ok(())
}



pub fn egg_to_serialized_egraph<L, A>(egraph: &egg::EGraph<L, A>) -> egraph_serialize::EGraph
where
    L: Language + Display,
    A: Analysis<L>,
{
    use egraph_serialize::*;
    let mut out = EGraph::default();
    for class in egraph.classes() {
        for (i, node) in class.nodes.iter().enumerate() {
            out.add_node(
                format!("{}.{}", class.id, i),
                Node {
                    op: node.to_string(),
                    children: node
                        .children()
                        .iter()
                        .map(|id| NodeId::from(format!("{}.0", id)))
                        .collect(),
                    eclass: ClassId::from(format!("{}", class.id)),
                    cost: Cost::new(1.0).unwrap(),
                },
            );
        }
    }
    out
}

pub fn process_json_prop_cost(json_str: &str) -> String {
    let mut data: Value = serde_json::from_str(&json_str).unwrap();

    if let Some(nodes) = data.get_mut("nodes").and_then(|nodes| nodes.as_object_mut()) {
        for node in nodes.values_mut() {
            let op = node["op"].as_str().unwrap();
            let cells = [("INVx11_ASAP7_75t_L", 303, 100, "Y=!A"), ("INVx13_ASAP7_75t_L", 350, 100, "Y=!A"), ("INVx1_ASAP7_75t_L", 70, 100, "Y=!A"), ("INVx2_ASAP7_75t_L", 93, 100, "Y=!A"), ("INVx3_ASAP7_75t_L", 117, 100, "Y=!A"), ("INVx4_ASAP7_75t_L", 140, 100, "Y=!A"), ("INVx5_ASAP7_75t_L", 163, 100, "Y=!A"), ("INVx6_ASAP7_75t_L", 187, 100, "Y=!A"), ("INVx8_ASAP7_75t_L", 233, 100, "Y=!A"), ("INVxp33_ASAP7_75t_L", 70, 100, "Y=!A"), ("INVxp67_ASAP7_75t_L", 70, 100, "Y=!A"), ("AND2x2_ASAP7_75t_L", 140, 100, "Y=(A * B)"), ("AND2x4_ASAP7_75t_L", 233, 100, "Y=(A * B)"), ("AND2x6_ASAP7_75t_L", 280, 100, "Y=(A * B)"), ("AND3x1_ASAP7_75t_L", 140, 100, "Y=(A * B * C)"), ("AND3x2_ASAP7_75t_L", 163, 100, "Y=(A * B * C)"), ("AND3x4_ASAP7_75t_L", 373, 100, "Y=(A * B * C)"), ("AND4x1_ASAP7_75t_L", 163, 100, "Y=(A * B * C * D)"), ("AND4x2_ASAP7_75t_L", 373, 100, "Y=(A * B * C * D)"), ("AND5x1_ASAP7_75t_L", 187, 100, "Y=(A * B * C * D * E)"), ("AND5x2_ASAP7_75t_L", 467, 100, "Y=(A * B * C * D * E)"), ("MAJIxp5_ASAP7_75t_L", 163, 100, "Y=(!A * !B) + (!A * !C) + (!B * !C)"), ("MAJx2_ASAP7_75t_L", 210, 100, "Y=(A * B) + (A * C) + (B * C)"), ("MAJx3_ASAP7_75t_L", 233, 100, "Y=(A * B) + (A * C) + (B * C)"), ("NAND2x1_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B)"), ("NAND2x1p5_ASAP7_75t_L", 187, 100, "Y=(!A) + (!B)"), ("NAND2x2_ASAP7_75t_L", 233, 100, "Y=(!A) + (!B)"), ("NAND2xp33_ASAP7_75t_L", 93, 100, "Y=(!A) + (!B)"), ("NAND2xp5_ASAP7_75t_L", 93, 100, "Y=(!A) + (!B)"), ("NAND2xp67_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B)"), ("NAND3x1_ASAP7_75t_L", 257, 100, "Y=(!A) + (!B) + (!C)"), ("NAND3x2_ASAP7_75t_L", 467, 100, "Y=(!A) + (!B) + (!C)"), ("NAND3xp33_ASAP7_75t_L", 117, 100, "Y=(!A) + (!B) + (!C)"), ("NAND4xp25_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND4xp75_ASAP7_75t_L", 327, 100, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND5xp2_ASAP7_75t_L", 163, 100, "Y=(!A) + (!B) + (!C) + (!D) + (!E)"), ("NOR2x1_ASAP7_75t_L", 140, 100, "Y=(!A * !B)"), ("NOR2x1p5_ASAP7_75t_L", 187, 100, "Y=(!A * !B)"), ("NOR2x2_ASAP7_75t_L", 233, 100, "Y=(!A * !B)"), ("NOR2xp33_ASAP7_75t_L", 93, 100, "Y=(!A * !B)"), ("NOR2xp67_ASAP7_75t_L", 140, 100, "Y=(!A * !B)"), ("NOR3x1_ASAP7_75t_L", 257, 100, "Y=(!A * !B * !C)"), ("NOR3x2_ASAP7_75t_L", 467, 100, "Y=(!A * !B * !C)"), ("NOR3xp33_ASAP7_75t_L", 117, 100, "Y=(!A * !B * !C)"), ("NOR4xp25_ASAP7_75t_L", 140, 100, "Y=(!A * !B * !C * !D)"), ("NOR4xp75_ASAP7_75t_L", 327, 100, "Y=(!A * !B * !C * !D)"), ("NOR5xp2_ASAP7_75t_L", 163, 100, "Y=(!A * !B * !C * !D * !E)"), ("OR2x2_ASAP7_75t_L", 140, 100, "Y=(A) + (B)"), ("OR2x4_ASAP7_75t_L", 187, 100, "Y=(A) + (B)"), ("OR2x6_ASAP7_75t_L", 280, 100, "Y=(A) + (B)"), ("OR3x1_ASAP7_75t_L", 140, 100, "Y=(A) + (B) + (C)"), ("OR3x2_ASAP7_75t_L", 163, 100, "Y=(A) + (B) + (C)"), ("OR3x4_ASAP7_75t_L", 210, 100, "Y=(A) + (B) + (C)"), ("OR4x1_ASAP7_75t_L", 163, 100, "Y=(A) + (B) + (C) + (D)"), ("OR4x2_ASAP7_75t_L", 187, 100, "Y=(A) + (B) + (C) + (D)"), ("OR5x1_ASAP7_75t_L", 187, 100, "Y=(A) + (B) + (C) + (D) + (E)"), ("OR5x2_ASAP7_75t_L", 210, 100, "Y=(A) + (B) + (C) + (D) + (E)"), ("XNOR2x1_ASAP7_75t_L", 280, 100, "Y=(A * B) + (!A * !B)"), ("XNOR2x2_ASAP7_75t_L", 257, 100, "Y=(A * B) + (!A * !B)"), ("XNOR2xp5_ASAP7_75t_L", 210, 100, "Y=(A * B) + (!A * !B)"), ("XOR2x1_ASAP7_75t_L", 280, 100, "Y=(A * !B) + (!A * B)"), ("XOR2x2_ASAP7_75t_L", 257, 100, "Y=(A * !B) + (!A * B)"), ("XOR2xp5_ASAP7_75t_L", 210, 100, "Y=(A * !B) + (!A * B)"), ("A2O1A1Ixp33_ASAP7_75t_L", 260, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!C)"), ("A2O1A1O1Ixp25_ASAP7_75t_L", 400, 100, "Y=(!A1 * !B * !D) + (!A2 * !B * !D) + (!C * !D)"), ("AO211x2_ASAP7_75t_L", 373, 100, "Y=(A1 * A2) + (B) + (C)"), ("AO21x1_ASAP7_75t_L", 140, 100, "Y=(A1 * A2) + (B)"), ("AO21x2_ASAP7_75t_L", 163, 100, "Y=(A1 * A2) + (B)"), ("AO221x1_ASAP7_75t_L", 233, 100, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO221x2_ASAP7_75t_L", 257, 100, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO222x2_ASAP7_75t_L", 513, 100, "Y=(A1 * A2) + (B1 * B2) + (C1 * C2)"), ("AO22x1_ASAP7_75t_L", 210, 100, "Y=(A1 * A2) + (B1 * B2)"), ("AO22x2_ASAP7_75t_L", 233, 100, "Y=(A1 * A2) + (B1 * B2)"), ("AO31x2_ASAP7_75t_L", 373, 100, "Y=(A1 * A2 * A3) + (B)"), ("AO322x2_ASAP7_75t_L", 350, 100, "Y=(A1 * A2 * A3) + (B1 * B2) + (C1 * C2)"), ("AO32x1_ASAP7_75t_L", 187, 100, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO32x2_ASAP7_75t_L", 210, 100, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO331x1_ASAP7_75t_L", 233, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO331x2_ASAP7_75t_L", 257, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO332x1_ASAP7_75t_L", 257, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO332x2_ASAP7_75t_L", 280, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO333x1_ASAP7_75t_L", 280, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO333x2_ASAP7_75t_L", 303, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO33x2_ASAP7_75t_L", 233, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3)"), ("AOI211x1_ASAP7_75t_L", 280, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI211xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI21x1_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp33_ASAP7_75t_L", 117, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp5_ASAP7_75t_L", 117, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI221x1_ASAP7_75t_L", 327, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI221xp5_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI222xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2)"), ("AOI22x1_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI311xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C) + (!A3 * !B * !C)"), ("AOI31xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI31xp67_ASAP7_75t_L", 303, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI321xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C) + (!A3 * !B1 * !C) + (!A3 * !B2 * !C)"), ("AOI322xp5_ASAP7_75t_L", 210, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2)"), ("AOI32xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2) + (!A3 * !B1) + (!A3 * !B2)"), ("AOI331xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B2 * !C1) + (!A1 * !B3 * !C1) + (!A2 * !B1 * !C1) + (!A2 * !B2 * !C1) + (!A2 * !B3 * !C1) + (!A3 * !B1 * !C1) + (!A3 * !B2 * !C1) + (!A3 * !B3 * !C1)"), ("AOI332xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2)"), ("AOI333xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B1 * !C3) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B2 * !C3) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A1 * !B3 * !C3) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B1 * !C3) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B2 * !C3) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A2 * !B3 * !C3) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B1 * !C3) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B2 * !C3) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2) + (!A3 * !B3 * !C3)"), ("AOI33xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A1 * !B3) + (!A2 * !B1) + (!A2 * !B2) + (!A2 * !B3) + (!A3 * !B1) + (!A3 * !B2) + (!A3 * !B3)"), ("O2A1O1Ixp33_ASAP7_75t_L", 0, 100, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("O2A1O1Ixp5_ASAP7_75t_L", 0, 100, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("OA211x2_ASAP7_75t_L", 187, 100, "Y=(A1 * B * C) + (A2 * B * C)"), ("OA21x2_ASAP7_75t_L", 163, 100, "Y=(A1 * B) + (A2 * B)"), ("OA221x2_ASAP7_75t_L", 373, 100, "Y=(A1 * B1 * C) + (A1 * B2 * C) + (A2 * B1 * C) + (A2 * B2 * C)"), ("OA222x2_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2)"), ("OA22x2_ASAP7_75t_L", 233, 100, "Y=(A1 * B1) + (A1 * B2) + (A2 * B1) + (A2 * B2)"), ("OA31x2_ASAP7_75t_L", 350, 100, "Y=(A1 * B1) + (A2 * B1) + (A3 * B1)"), ("OA331x1_ASAP7_75t_L", 233, 100, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA331x2_ASAP7_75t_L", 257, 100, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA332x1_ASAP7_75t_L", 257, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA332x2_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA333x1_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA333x2_ASAP7_75t_L", 303, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA33x2_ASAP7_75t_L", 233, 100, "Y=(A1 * B1) + (A1 * B2) + (A1 * B3) + (A2 * B1) + (A2 * B2) + (A2 * B3) + (A3 * B1) + (A3 * B2) + (A3 * B3)"), ("OAI211xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B) + (!C)"), ("OAI21x1_ASAP7_75t_L", 187, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp33_ASAP7_75t_L", 117, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp5_ASAP7_75t_L", 117, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI221xp5_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C)"), ("OAI222xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI22x1_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI311xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2 * !A3) + (!B1) + (!C1)"), ("OAI31xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI31xp67_ASAP7_75t_L", 303, 100, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI321xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C)"), ("OAI322xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI32xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2)"), ("OAI331xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1)"), ("OAI332xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2)"), ("OAI333xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)"), ("OAI33xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)")];

            let ops = ["dff","nor","and","nand","not","or","xor"];

            let new_cost = match op {
                op if cells.iter().any(|&(name, _, _, _)| name == op) => {
                    cells.iter().find(|&&(name, _, _, _)| name == op).unwrap().1
                },
                op if ops.contains(&op) => 500000,
                _ => 0,
            };
            
            node["cost"] = serde_json::to_value(new_cost).unwrap();
        }
    }

    serde_json::to_string_pretty(&data).unwrap()
}

// pub fn save_serialized_egraph_to_json(serialized_egraph: &SerializedEGraph, file_path: &PathBuf, root_ids: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
//     let file = File::create(&file_path)?;
//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, &serialized_egraph)?;

//     let root_eclasses_value: serde_json::Value = root_ids
//         .iter()
//         .map(|id| serde_json::Value::String(id.to_string()))
//         .collect();

//     let json_string = std::fs::read_to_string(&file_path)?;
//     let mut json_data: serde_json::Value = serde_json::from_str(&json_string)?;
//     json_data["root_eclasses"] = root_eclasses_value;

//     let file = File::create(&file_path)?;
//     let writer = BufWriter::new(file);
//     serde_json::to_writer_pretty(writer, &json_data)?;

//     Ok(())
// }

fn save_json(egraph: EGraph<SimpleLanguage, ()>,root_ids:&[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let output_egraph_json_path = env::current_dir().unwrap().join("json/egraph.json");
    save_egraph_to_json(&egraph, &output_egraph_json_path);

    let serialized_output_egraph = egg_to_serialized_egraph(&egraph);
    let json_string = serde_json::to_string(&serialized_output_egraph).unwrap();
    let cost_string = process_json_prop_cost(&json_string);
    let mut json_data: serde_json::Value = serde_json::from_str(&cost_string)?;
    json_data["root_eclasses"] = serde_json::Value::Array(root_ids.iter().map(|id| serde_json::Value::String(id.to_string())).collect());
    let output_egraph_cost_json_path = env::current_dir().unwrap().join("json/serialized_egraph.json");
    let file = File::create(&output_egraph_cost_json_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &json_data);
    Ok(())
}


fn build_expr_from_selection(runner: &Runner<SimpleLanguage, ()>, result: &extraction_gym::ExtractionResult) -> Option<RecExpr<SimpleLanguage>> {
    let mut str_choice: IndexMap<String, &str> = IndexMap::default();
    for (classid, nodeid) in &result.choices {
        str_choice.insert(classid.as_ref().to_string(), nodeid.as_ref().split('.').last()?);
    }

    let mut id2node: IndexMap<Id, SimpleLanguage> = IndexMap::default();
    for class in runner.egraph.classes() {
        let class_id = class.id;
        let string_classid = class_id.to_string();
        let selected_id_str = str_choice.get(&string_classid)?;
        let num_id: usize = selected_id_str.parse().ok()?;
        let selected_node = class.nodes.get(num_id)?;
        id2node.insert(class_id, selected_node.clone());
    }

    let root_id = runner.roots.get(0)?;
    let root = id2node.get(root_id)?;
    let expr = root.build_recexpr(|id| id2node.get(&id).expect("Id not selected").clone());

    Some(expr)
}


fn build_expr(mode: &str, vertices:Vec<String>, in_pairs:HashMap<String, String>,edge_pairs:HashMap<String, Vec<String>>, input:Vec<String>, output:Vec<String>) -> RecExpr<SimpleLanguage>{
    let mut expr: RecExpr<SimpleLanguage> = RecExpr::default();
    let mut vertices_map: HashMap<String, Id> = HashMap::new();
    let mut queue: Vec<String> = vertices.clone();
    let mut out_id: HashMap<String, Id> = HashMap::new();
    queue.reverse();
    while !queue.is_empty(){
        let operation=queue.pop().unwrap();
        if let Some(input_edges) = in_pairs.get(&operation) {
            let mut in_id = Vec::new();
            for vertex in edge_pairs.get(input_edges).unwrap_or(&Vec::new()){
                if let Some(id) = vertices_map.get(vertex) {
                    in_id.push(*id);
                }
            }

            let name: Vec<&str> = match mode {
                "bench" => input_edges.split('_').collect(),
                "blif" => {
                    if let Some(index) = input_edges.rfind('_') {
                        let first_part = &input_edges[..index];
                        let last_part = &input_edges[index + 1..];
                        vec![first_part, last_part]
                    } else {
                        vec![input_edges]  // No underscore found, treat the whole string as one part
                    }
                },
                _ => panic!("Unknown mode: {}", mode),  // Panic for any other mode
            };
            // let name: Vec<&str> = input_edges.split("_").collect();

            
            if name.len()==2 {
                let stripped_name = match mode {
                    "bench" => name.first().expect("The list is empty").to_lowercase(),
                    "blif" => name.first().expect("The list is empty").to_string(),
                    _ => panic!("This should not happen as mode is checked earlier.")
                };
                if let Some(value) = convert_to_simple_language_enum (in_id, &stripped_name) {
                    let temp_id = expr.add(value);
                    if output.contains(&operation){
                        out_id.insert(operation.clone(), temp_id);
                    }
                    vertices_map.insert(operation.clone(), temp_id);
                    queue.retain(|x| x != &operation);
                }
                else {
                    panic!("Unknown enum variant for {}", stripped_name);
                }
            }
            else {
                panic!("Edge format error: {}", input_edges);
            }
        } else {
            let temp_id = expr.add(SimpleLanguage::Symbol(operation.clone().into()));
            if output.contains(&operation){
                out_id.insert(operation.clone(), temp_id);
            }
            vertices_map.insert(operation.clone(), temp_id);
            queue.retain(|x| x != &operation);
        }
    }

    let mut vec_out_id: Vec<Id> = Vec::new();
    for element in &output{
        vec_out_id.push(out_id[element]);
    }

    let value = SimpleLanguage::OUT(vec_out_id);
    let temp_id = expr.add(value);
    vertices_map.insert("output".to_string(), temp_id);

    expr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        let path1="benchmark/iscas85/c432.bench";
        let path2="benchmark/output/c432/c432_bottom_up.blif";
        let path3="benchmark/output/c432/c432_faster_bottom_up.blif";
        // lib::parse_blif(&path2);
        match (lib::parse_graph(&path1),lib::parse_blif(&path2),lib::parse_blif(&path3)) {
            (Ok((vertices1, _, in_pairs1, _ ,edge_pairs1, input1, output1)),Ok((vertices2, _, in_pairs2, _ ,edge_pairs2, input2, output2)),Ok((vertices3, _, in_pairs3, _ ,edge_pairs3, input3, output3))) => {
                let expr1 = build_expr("bench",vertices1, in_pairs1, edge_pairs1, input1, output1);
                let expr2 = build_expr("blif",vertices2, in_pairs2, edge_pairs2, input2, output2);
                let expr3 = build_expr("blif",vertices3, in_pairs3, edge_pairs3, input3, output3);
                let runner = Runner::default().with_time_limit(Duration::from_secs(1000000000)).with_iter_limit(1000000000).with_node_limit(70000).with_expr(&expr1).run(&static_rules::make_rules());
                println!(" Stop Reason: {:?}",runner.stop_reason);
                let equivs1 = runner.egraph.equivs(&expr1, &expr2).len();
                let equivs2 = runner.egraph.equivs(&expr1, &expr3).len();
                if equivs1 == 0 {
                    println!("Not equal\n{}\n{}",path1,path2);
                }
                else {
                    println!("Equal\n{}\n{}",path1,path2);
                }
                if equivs2 == 0 {
                    println!("Not equal\n{}\n{}",path1,path3);
                }
                else {
                    println!("Equal\n{}\n{}",path1,path3);
                }
            },
            _ => {
                // Handle the error here
                println!("Failed to parse the graph");
            }
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_equal() {
//         let args: Vec<String> = env::args().collect();
//         if args.len() < 3 {
//             eprintln!("Usage: {} <benchmark>", args[0]);
//             std::process::exit(1);
//         }
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let if_print = false;
    if args.len() < 2 {
        eprintln!("Usage: {} <benchmark>", args[0]);
        std::process::exit(1);
    }
    let benchmark = &args[1];
    // let benchmark = "c2670";
    let path = "benchmark/iscas85/";
    let blif_dir_path = &format!("benchmark/output/{}/",benchmark);

    // check and create the folder
    let blif_dir = Path::new(&blif_dir_path);
    if !blif_dir.exists() {
        if let Err(e) = fs::create_dir_all(blif_dir) {
            eprintln!("Error creating directory {}: {}", blif_dir_path, e);
            std::process::exit(1);
        }
    }

    // match lib::parse_graph("benchmark/iscas85/c7552.bench") {
    let input_file_path=format!("{}{}.bench", path, benchmark);
    match lib::parse_graph(&input_file_path) {
        Ok((vertices, _, in_pairs, _ ,edge_pairs, input, output)) => {
            let expr = build_expr("bench",vertices, in_pairs, edge_pairs, input.clone(), output.clone());
            if if_print {println!("Start E-graph Grownth");}
            let start = Instant::now();
            let runner = Runner::default().with_time_limit(Duration::from_secs(1000000000)).with_iter_limit(1000000000).with_node_limit(70000).with_expr(&expr).run(&static_rules::make_rules());
            if if_print {println!(" Stop Reason: {:?}",runner.stop_reason);}
            let grownth_duration = start.elapsed();
            if if_print {println!("      Saturation: {:?}",grownth_duration);}

            let root_vec: Vec<usize> = runner.roots.iter().map(|&id| id.into()).collect();
            let root: &[usize] = &root_vec;
            save_json(runner.egraph.clone(),root);

            let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
            extractors.retain(|_, ed| ed.get_use_for_bench());
            let filename: String = "json/serialized_egraph.json".into();
            let serialized_egraph = SerializedEGraph::from_json_file(&filename)
            .with_context(|| format!("Failed to parse {filename}"))
            .unwrap();

            /*------------------faster-greedy-dag----------------*/
            let start = Instant::now();
            let extractor_name: String = "faster-greedy-dag".into();
            let ed = extractors
            .get(extractor_name.as_str())
            .with_context(|| format!("Unknown extractor: {extractor_name}"))
            .unwrap();
        
            let result = ed.get_extractor().extract(&serialized_egraph, &serialized_egraph.root_eclasses);
            result.check(&serialized_egraph);

            let expr:Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&runner, &result);
            match expr {
                Some(expr) => {
                    let blif_path = &format!("benchmark/output/{}/{}_faster_greedy_dag.blif",benchmark,benchmark);
                    if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &expr, input.clone(), output.clone()) {
                        eprintln!("Failed to parse to Blif:{}",e);
                    }
                },
                None => panic!("An error occurred, but no details are available."),
            }
            // println!("{}",egraph.classes());
            // println!("{:?}",result.choices);



            let tree = result.tree_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let dag = result.dag_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let grownth_duration = start.elapsed();
            if if_print {
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "Faster-greedy-dag", grownth_duration, tree, dag);
            }
            else{
                print!("{}|",dag)
            }
            /*------------------------------------------------------------*/

            /*------------------bottom-up----------------*/
            let start = Instant::now();
            let extractor_name: String = "bottom-up".into();
            let ed = extractors
            .get(extractor_name.as_str())
            .with_context(|| format!("Unknown extractor: {extractor_name}"))
            .unwrap();
        
            let result: extraction_gym::ExtractionResult = ed.get_extractor().extract(&serialized_egraph, &serialized_egraph.root_eclasses);
            result.check(&serialized_egraph);

            let expr:Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&runner, &result);
            match expr {
                Some(expr) => {
                    let blif_path = &format!("benchmark/output/{}/{}_bottom_up.blif",benchmark,benchmark);
                    if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &expr, input.clone(), output.clone()) {
                        eprintln!("Failed to parse to Blif:{}",e);
                    }
                },
                None => panic!("An error occurred, but no details are available."),
            }


            let tree = result.tree_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let dag = result.dag_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let grownth_duration = start.elapsed();
            if if_print {
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "Bottom-up", grownth_duration, tree, dag);
            }
            else{
                print!("{}|",dag)
            }
            /*------------------------------------------------------------*/

            /*------------------faster-bottom-up----------------*/
            let start = Instant::now();
            let extractor_name: String = "faster-bottom-up".into();
            let ed = extractors
            .get(extractor_name.as_str())
            .with_context(|| format!("Unknown extractor: {extractor_name}"))
            .unwrap();
        
            let result = ed.get_extractor().extract(&serialized_egraph, &serialized_egraph.root_eclasses);
            result.check(&serialized_egraph);

            let expr:Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&runner, &result);
            match expr {
                Some(expr) => {
                    let blif_path = &format!("benchmark/output/{}/{}_faster_bottom_up.blif",benchmark,benchmark);
                    if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &expr, input.clone(), output.clone()) {
                        eprintln!("Failed to parse to Blif:{}",e);
                    }
                },
                None => panic!("An error occurred, but no details are available."),
            }

            let tree = result.tree_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let dag = result.dag_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let grownth_duration = start.elapsed();
            if if_print {
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "faster-bottom-up", grownth_duration, tree, dag);
            }
            else{
                print!("{}|",dag)
            }
            /*------------------------------------------------------------*/

            /*------------------global-greedy-dag----------------*/
            let start = Instant::now();
            let extractor_name: String = "global-greedy-dag".into();
            let ed = extractors
            .get(extractor_name.as_str())
            .with_context(|| format!("Unknown extractor: {extractor_name}"))
            .unwrap();
        
            let result = ed.get_extractor().extract(&serialized_egraph, &serialized_egraph.root_eclasses);
            result.check(&serialized_egraph);

            let expr:Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&runner, &result);
            match expr {
                Some(expr) => {
                    let blif_path = &format!("benchmark/output/{}/{}_global_greedy_dag.blif",benchmark,benchmark);
                    if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &expr, input.clone(), output.clone()) {
                        eprintln!("Failed to parse to Blif:{}",e);
                    }
                },
                None => panic!("An error occurred, but no details are available."),
            }

            let tree = result.tree_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let dag = result.dag_cost(&serialized_egraph, &serialized_egraph.root_eclasses);
            let grownth_duration = start.elapsed();
            if if_print {
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "global-greedy-dag", grownth_duration, tree, dag);
            }
            else{
                print!("{}|",dag)
            }
            /*------------------------------------------------------------*/

            #[cfg(feature = "ilp-cbc")]
            {
                /*------------------ilp-cbc-timeout----------------*/
                let start = Instant::now();
                let extractor_name: String = "ilp-cbc-timeout".into();
                let ed = extractors
                .get(extractor_name.as_str())
                .with_context(|| format!("Unknown extractor: {extractor_name}"))
                .unwrap();
            
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);

                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                if if_print {
                    println!("{:<18}: runtime-{:?} tree-{} dag-{}", "ilp-cbc-timeout", grownth_duration, tree, dag);
                }
                else{
                    print!("{}|",dag)
                }
                /*------------------------------------------------------------*/


                /*------------------faster-ilp-cbc-timeout----------------*/
                let start = Instant::now();
                let extractor_name: String = "faster-ilp-cbc-timeout".into();
                let ed = extractors
                .get(extractor_name.as_str())
                .with_context(|| format!("Unknown extractor: {extractor_name}"))
                .unwrap();
            
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);

                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                if if_print {
                    println!("{:<18}: runtime-{:?} tree-{} dag-{}", "faster-ilp-cbc-timeout", grownth_duration, tree, dag);
                }
                else{
                    print!("{}|",dag)
                }
                /*------------------------------------------------------------*/

                /*------------------faster-ilp-cbc----------------*/
                // let start = Instant::now();
                // let extractor_name: String = "faster-ilp-cbc".into();
                // let ed = extractors
                // .get(extractor_name.as_str())
                // .with_context(|| format!("Unknown extractor: {extractor_name}"))
                // .unwrap();
            
                // let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                // result.check(&egraph);

                // let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                // let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                // let grownth_duration = start.elapsed();
                // println!("{:<18}: runtime-{:?} tree-{} dag-{}", "faster-ilp-cbc", grownth_duration, tree, dag);
                /*------------------------------------------------------------*/
            }
        
            // for (key, value) in result.choices.iter() {
            //     println!("{}: {}", key, value);
            // }
        


            result.check(&serialized_egraph);
            
        
        },
        Err(e) => {
            // Handle the error here
            println!("Failed to parse the graph: {}", e);
        }
    }
}