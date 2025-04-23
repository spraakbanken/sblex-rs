use crate::ports::{LookupLid, SblexService};

#[derive(Debug, Clone)]
pub struct Service<L> {
    lookup_lid: L,
}

impl<L> Service<L>
where
    L: LookupLid,
{
    pub fn new(lookup_lid: L) -> Self {
        Self { lookup_lid }
    }
}

impl<L> SblexService for Service<L>
where
    L: LookupLid,
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
}
