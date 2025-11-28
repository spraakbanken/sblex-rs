use hashbrown::HashSet;

use crate::models::{
    fullform::Fullform, fullform_lex::FullformLex, lemma::Lemma, lexeme::Lexeme,
    lookup::LookupError,
};

pub trait SblexService: Clone + Send + Sync + 'static {
    fn lookup_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn lookup_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
    fn lookup_morph(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError>;
    fn lookup_morph_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError>;
    fn fullform_lex_query(&self, segment: &str) -> Result<Vec<FullformLex>, LookupError> {
        let mut lemmas = HashSet::new();
        for x in self.lookup_ff(segment)? {
            if !["ci", "cm", "c"].contains(&x.msd.as_str()) && !x.pos.ends_with('h') {
                let Fullform { id, gf, p, .. } = x;
                lemmas.insert((id, gf, p));
            }
        }
        let mut result = Vec::with_capacity(lemmas.len());
        for (lem, gf, p) in lemmas {
            let lexemes = self.lookup_lemma(&lem)?.unwrap().into_lexemes();
            for lex in lexemes {
                let Lexeme { fm, fp, .. } = self.lookup_lexeme(&lex)?.unwrap();
                let fps: Vec<String> = fp.split(' ').map(ToString::to_string).collect();
                result.push(FullformLex {
                    id: lex,
                    fm,
                    fp: fps,
                    l: lem.clone(),
                    gf: gf.clone(),
                    p: p.clone(),
                });
            }
        }
        Ok(result)
    }
    fn lookup_ff(&self, segment: &str) -> Result<Vec<Fullform>, LookupError> {
        if let Some(buf) = self.lookup_morph(segment)? {
            match serde_json::from_slice(&buf) {
                Ok(ffs) => Ok(ffs),
                Err(err) => Err(LookupError::Unknown(Box::new(err))),
            }
        } else {
            Ok(Vec::new())
        }
    }
}

pub trait LookupLid: Clone + Send + Sync + 'static {
    fn get_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn get_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
}

pub trait Morphology: Clone + Send + Sync + 'static {
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError>;
    fn lookup_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError>;
}
