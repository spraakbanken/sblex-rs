use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead},
    iter::once,
    path::Path,
};

use hashbrown::HashMap;

use crate::{
    models::{lemma::Lemma, lexeme::Lexeme},
    ports::LookupLid,
};

#[derive(Debug, Clone)]
pub struct MemLookupLid {
    lex_map: HashMap<String, Lexeme>,
    lem_map: HashMap<String, Lemma>,
}

impl MemLookupLid {
    pub fn from_tsv_path(tsv_path: &Path) -> Result<Self, io::Error> {
        let mut lem_map: HashMap<String, (BTreeSet<String>, String, String)> = HashMap::new();
        let mut lex_map: HashMap<
            String,
            (
                String,
                String,
                BTreeSet<String>,
                BTreeSet<String>,
                BTreeSet<String>,
            ),
        > = HashMap::new();
        let file = File::open(tsv_path)?;
        let reader = io::BufReader::new(file);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            // let line: Vec<String> = line.split('\t').map(String::from).collect();
            let line: Vec<&str> = line.split('\t').collect();
            if line.len() != 7 {
                tracing::warn!(
                    "Expected 7 columns: Line {} has {} columns, skipping '{:?}'",
                    i,
                    line.len(),
                    line
                );
                continue;
            }
            let lexeme = line[0];
            let mother = line[1];
            let fathers = line[2];
            let lemma = line[3];
            let gf = line[4];
            let pos = line[5];
            let p = line[6];
            // create lemma-lexeme mappings
            lem_map
                .entry_ref(lemma)
                .and_modify(|(s, _p, _gf)| {
                    s.insert(lexeme.into());
                })
                .or_insert_with(|| {
                    (
                        BTreeSet::from_iter(once(lexeme.into())),
                        p.trim().into(),
                        gf.into(),
                    )
                });
            // add mother, father and lemma
            lex_map
                .entry_ref(lexeme)
                .and_modify(|(m, f, _mf, _gf, ls)| {
                    // we may have added the children already
                    *m = mother.into();
                    *f = fathers.into();
                    ls.insert(lemma.into());
                })
                .or_insert_with(|| {
                    (
                        mother.into(),
                        fathers.into(),
                        BTreeSet::new(),
                        BTreeSet::new(),
                        BTreeSet::from_iter(once(lemma.into())),
                    )
                });

            // add m-children
            lex_map
                .entry_ref(mother)
                .and_modify(|(_m, _f, mf, _pf, _)| {
                    mf.insert(lexeme.into());
                })
                .or_insert_with(|| {
                    // we don't know the mother or the father yet
                    (
                        "".into(),
                        "".into(),
                        BTreeSet::from_iter(once(lexeme.into())),
                        BTreeSet::new(),
                        BTreeSet::new(),
                    )
                });

            // add p-children
            for father in fathers.split(' ') {
                lex_map
                    .entry_ref(father)
                    .and_modify(|(_m, _f, _mf, pf, _)| {
                        pf.insert(lexeme.into());
                    })
                    .or_insert_with(|| {
                        // we don't know the mother or the father yet
                        (
                            "".into(),
                            "".into(),
                            BTreeSet::new(),
                            BTreeSet::from_iter(once(lexeme.into())),
                            BTreeSet::new(),
                        )
                    });
            }
        }
        let mut path_map: HashMap<String, Vec<String>> = HashMap::new();
        // add path
        for sense in lex_map.keys() {
            let mut pth = Vec::new();
            let mut sns = sense;
            while sns != "PRIM..1" && sns != "" && !pth.contains(sns) {
                if let Some((primary, _, _, _, _)) = lex_map.get(sns) {
                    sns = primary;
                    if sns != "PRIM..1" {
                        pth.push(sns.clone());
                    }
                }
            }
            path_map.insert(sense.into(), pth);
        }

        let mut final_lem_map = HashMap::new();
        for (lem, (s, p, gf)) in lem_map.drain() {
            final_lem_map.insert(lem, Lemma::new(p, gf, s.into_iter().collect()));
        }

        let mut final_lex_map = HashMap::new();
        for (lex, (m, f, mchildren, pchildren, lemmas)) in lex_map.drain() {
            let path = path_map[&lex].clone();
            let ppath = father_path(&f, &path_map);
            final_lex_map.insert(
                lex.clone(),
                Lexeme::new(
                    lex,
                    m,
                    f,
                    mchildren.into_iter().collect(),
                    pchildren.into_iter().collect(),
                    lemmas.into_iter().collect(),
                    path,
                    ppath,
                ),
            );
        }
        Ok(Self {
            lex_map: final_lex_map,
            lem_map: final_lem_map,
        })
    }
}

fn father_path(fathers: &str, path_map: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for s in fathers.split(' ') {
        if s != "PRIM..1" && s != "" {
            result.push(path_map[s].clone());
        }
    }
    result
}

impl LookupLid for MemLookupLid {
    fn get_lemma(&self, lid: &str) -> Result<Option<Lemma>, crate::models::lookup::LookupError> {
        Ok(self.lem_map.get(lid).cloned())
    }

    fn get_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, crate::models::lookup::LookupError> {
        Ok(self.lex_map.get(lid).cloned())
    }
}
