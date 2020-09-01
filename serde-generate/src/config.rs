// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::{BTreeMap, BTreeSet};

/// Code generation options meant to be supported by all languages.
#[derive(Clone, Debug)]
pub struct CodeGeneratorConfig {
    pub(crate) module_name: String,
    pub(crate) serialization: bool,
    pub(crate) encodings: BTreeSet<Encoding>,
    pub(crate) external_definitions: ExternalDefinitions,
    pub(crate) comments: DocComments,
}

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Encoding {
    Bincode,
    Lcs,
}

/// Track types definitions provided by external modules.
pub type ExternalDefinitions =
    std::collections::BTreeMap</* module */ String, /* type names */ Vec<String>>;

/// Track documentation to be attached to particular definitions.
pub type DocComments =
    std::collections::BTreeMap</* qualified name */ Vec<String>, /* comment */ String>;

/// How to copy generated source code and available runtimes for a given language.
pub trait SourceInstaller {
    type Error;

    /// Create a module exposing the container types contained in the registry.
    fn install_module(
        &self,
        config: &CodeGeneratorConfig,
        registry: &serde_reflection::Registry,
    ) -> std::result::Result<(), Self::Error>;

    /// Install the serde runtime.
    fn install_serde_runtime(&self) -> std::result::Result<(), Self::Error>;

    /// Install the bincode runtime.
    fn install_bincode_runtime(&self) -> std::result::Result<(), Self::Error>;

    /// Install the Libra Canonical Serialization (LCS) runtime.
    fn install_lcs_runtime(&self) -> std::result::Result<(), Self::Error>;
}

impl CodeGeneratorConfig {
    /// Default config for the given module name.
    pub fn new(module_name: String) -> Self {
        Self {
            module_name,
            serialization: true,
            encodings: BTreeSet::new(),
            external_definitions: BTreeMap::new(),
            comments: BTreeMap::new(),
        }
    }

    /// Whether to include serialization methods.
    pub fn with_serialization(mut self, serialization: bool) -> Self {
        self.serialization = serialization;
        self
    }

    /// Whether to include specialized methods for specific encodings.
    pub fn with_encodings<I>(mut self, encodings: I) -> Self
    where
        I: IntoIterator<Item = Encoding>,
    {
        self.encodings = encodings.into_iter().collect();
        self
    }

    /// Container names provided by external modules.
    pub fn with_external_definitions(mut self, external_definitions: ExternalDefinitions) -> Self {
        self.external_definitions = external_definitions;
        self
    }

    /// Comments attached to particular entity.
    pub fn with_comments(mut self, mut comments: DocComments) -> Self {
        // Make sure comments end with a (single) newline.
        for comment in comments.values_mut() {
            *comment = format!("{}\n", comment.trim());
        }
        self.comments = comments;
        self
    }
}

impl Encoding {
    pub fn name(self) -> &'static str {
        match self {
            Encoding::Bincode => "bincode",
            Encoding::Lcs => "lcs",
        }
    }
}