define_rpd_ome!(
    /// Gluon-gluon operator matrix element A_{gg,Q}.
    ///
    /// This OME has regular, plus, and delta parts. It describes the
    /// gluon-to-gluon transition through a heavy-quark loop.
    ///
    /// # Example
    ///
    /// ```
    /// use libome_rs::AggQ;
    ///
    /// let val = AggQ::reg(0.25, -5.0, 3.0, 0.5);
    /// assert!(val.is_finite());
    /// ```
    AggQ,
    AggQ
);
