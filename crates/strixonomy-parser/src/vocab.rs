use oxigraph::model::NamedNodeRef;

pub struct OWL;

impl OWL {
    pub fn class() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#Class")
    }

    pub fn object_property() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#ObjectProperty")
    }

    pub fn datatype_property() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#DatatypeProperty")
    }

    pub fn annotation_property() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#AnnotationProperty")
    }

    pub fn ontology() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#Ontology")
    }

    pub fn named_individual() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#NamedIndividual")
    }

    pub fn imports() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#imports")
    }

    pub fn version_iri() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#versionIRI")
    }

    pub fn deprecated() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#deprecated")
    }

    pub fn same_as() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2002/07/owl#sameAs")
    }
}

pub struct Rdfs;

impl Rdfs {
    pub fn label() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#label")
    }

    pub fn comment() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#comment")
    }

    pub fn class() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#Class")
    }

    pub fn sub_class_of() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#subClassOf")
    }

    pub fn datatype() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#Datatype")
    }
}

pub struct Rdf;

impl Rdf {
    pub fn type_() -> NamedNodeRef<'static> {
        NamedNodeRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")
    }
}
