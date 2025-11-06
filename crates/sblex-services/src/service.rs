use crate::ports::{LookupLid, Morphology, SblexService};

#[derive(Debug, Clone)]
pub struct Service<L, M> {
    lookup_lid: L,
    morphology: M,
}

impl<L, M> Service<L, M>
where
    L: LookupLid,
    M: Morphology,
{
    pub fn new(lookup_lid: L, morphology: M) -> Self {
        Self {
            lookup_lid,
            morphology,
        }
    }
}

impl<L, M> SblexService for Service<L, M>
where
    L: LookupLid,
    M: Morphology,
{
    fn lookup_lemma(
        &self,
        lid: &str,
    ) -> Result<Option<crate::models::lemma::Lemma>, crate::models::lookup::LookupError> {
        self.lookup_lid.get_lemma(lid)
    }
    fn lookup_lexeme(
        &self,
        lid: &str,
    ) -> Result<Option<crate::models::lexeme::Lexeme>, crate::models::lookup::LookupError> {
        self.lookup_lid.get_lexeme(lid)
    }
    fn lookup_morph(
        &self,
        fragment: &str,
    ) -> Result<Option<Vec<u8>>, crate::models::lookup::LookupError> {
        self.morphology.lookup(fragment)
    }

    fn lookup_morph_with_cont(
        &self,
        fragment: &str,
    ) -> Result<Vec<u8>, crate::models::lookup::LookupError> {
        self.morphology.lookup_with_cont(fragment)
    }
}
