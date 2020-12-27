use crate::bail_status as bail;
use std::fmt::{self, Display};
use std::str::FromStr;

/// A relationship passed as part of a [`LinkDirective`].
///
/// # Specifications
///
/// - [RFC 5988, section 6.2.2: Initial Registry Contents](https://tools.ietf.org/html/rfc5988#section-6.2.2)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RelationType {
    /// Designates a substitute for the link's context.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Alternate,

    /// Refers to an appendix.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Appendix,

    /// Refers to a bookmark or entry point.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Bookmark,

    /// Refers to a chapter in a collection of resources.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Chapter,

    /// Refers to a table of contents.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Contents,

    /// Refers to a copyright statement that applies to the
    /// link's context.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Copyright,

    /// Refers to a resource containing the most recent
    /// item(s) in a collection of resources.
    ///
    /// # References
    ///
    /// - [RFC5005](https://tools.ietf.org/html/RFC5005)
    Current,

    /// Refers to a resource providing information about the
    /// link's context.
    ///
    /// # References
    ///
    /// - [Documentation: <http://www.w3.org/TR/powder-dr/#assoc-linking>](https://tools.ietf.org/html/Documentation: <http://www.w3.org/TR/powder-dr/#assoc-linking>)
    Describedby,

    /// Refers to a resource that can be used to edit the
    /// link's context.
    ///
    /// # References
    ///
    /// - [RFC5023](https://tools.ietf.org/html/RFC5023)
    Edit,

    /// Refers to a resource that can be used to edit media
    /// associated with the link's context.
    ///
    /// # References
    ///
    /// - [RFC5023](https://tools.ietf.org/html/RFC5023)
    EditMedia,

    /// Identifies a related resource that is potentially
    /// large and might require special handling.
    ///
    /// # References
    ///
    /// - [RFC4287](https://tools.ietf.org/html/RFC4287)
    Enclosure,

    /// An IRI that refers to the furthest preceding resource
    /// in a series of resources.
    ///
    /// # Notes
    ///
    /// this relation type registration did not indicate a
    /// reference.  Originally requested by Mark Nottingham in December
    /// 2004.
    ///
    /// # References
    ///
    /// - [RFC5988](https://tools.ietf.org/html/RFC5988)
    First,

    /// Refers to a glossary of terms.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Glossary,

    /// Refers to a resource offering help (more information,
    /// links to other sources information, etc.)
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Help,

    /// Refers to a hub that enables registration for
    /// notification of updates to the context.
    ///
    /// # Notes
    ///
    /// this relation type was requested by Brett Slatkin.
    ///
    /// # References
    ///
    /// - [http://pubsubhubbub.googlecode.com](http://pubsubhubbub.googlecode.com/svn/trunk/pubsubhubbub-core-0.3.html)
    Hub,

    /// Refers to an index.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Index,

    /// An IRI that refers to the furthest following resource
    /// in a series of resources.
    ///
    /// # Notes
    ///
    /// this relation type registration did not indicate a
    /// reference.  Originally requested by Mark Nottingham in December
    /// 2004.
    ///
    /// # References
    ///
    /// - [RFC5988](https://tools.ietf.org/html/RFC5988)
    Last,

    /// Points to a resource containing the latest (e.g.,
    /// current) version of the context.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    LatestVersion,

    /// Refers to a license associated with the link's
    /// context.
    ///
    /// # References
    ///
    /// - [RFC4946](https://tools.ietf.org/html/RFC4946)
    License,

    /// Refers to the next resource in a ordered series of
    /// resources.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Next,

    /// Refers to the immediately following archive resource.
    ///
    /// # References
    ///
    /// - [RFC5005](https://tools.ietf.org/html/RFC5005)
    NextArchive,

    /// indicates a resource where payment is accepted.
    ///
    /// # Notes
    ///
    /// This relation type registration did not indicate a
    /// reference.  Requested by Joshua Kinberg and Robert Sayre.  It is
    /// meant as a general way to facilitate acts of payment, and thus
    /// this specification makes no assumptions on the type of payment or
    /// transaction protocol.  Examples may include a Web page where
    /// donations are accepted or where goods and services are available
    /// for purchase. rel="payment" is not intended to initiate an
    /// automated transaction.  In Atom documents, a link element with a
    /// rel="payment" attribute may exist at the feed/channel level and/or
    /// the entry/item level.  For example, a rel="payment" link at the
    /// feed/channel level may point to a "tip jar" URI, whereas an entry/
    /// item containing a book review may include a rel="payment" link
    /// that points to the location where the book may be purchased
    /// through an online retailer.
    ///
    /// # References
    ///
    /// - [RFC5988](https://tools.ietf.org/html/RFC5988)
    Payment,

    /// Refers to the previous resource in an ordered series
    /// of resources.  Synonym for "previous".
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Prev,

    /// Points to a resource containing the predecessor
    /// version in the version history.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    PredecessorVersion,

    /// Refers to the previous resource in an ordered series
    /// of resources.  Synonym for "prev".
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Previous,

    /// Refers to the immediately preceding archive resource.
    ///
    /// # References
    ///
    /// - [RFC5005](https://tools.ietf.org/html/RFC5005)
    PrevArchive,

    /// Identifies a related resource.
    ///
    /// # References
    ///
    /// - [RFC4287](https://tools.ietf.org/html/RFC4287)
    Related,

    /// Identifies a resource that is a reply to the context
    /// of the link.
    ///
    /// # References
    ///
    /// - [RFC4685](https://tools.ietf.org/html/RFC4685)
    Replies,

    /// Refers to a section in a collection of resources.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Section,

    /// Conveys an identifier for the link's context.
    ///
    /// # References
    ///
    /// - [RFC4287](https://tools.ietf.org/html/RFC4287)
    Self_,

    /// Indicates a URI that can be used to retrieve a
    /// service document.
    ///
    /// # Notes
    ///
    /// When used in an Atom document, this relation type specifies
    /// Atom Publishing Protocol service documents by default.  Requested
    /// by James Snell.
    ///
    /// # References
    ///
    /// - [RFC5023](https://tools.ietf.org/html/RFC5023)
    Service,

    /// Refers to the first resource in a collection of
    /// resources.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Start,

    /// Refers to an external style sheet.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Stylesheet,

    /// Refers to a resource serving as a subsection in a
    /// collection of resources.
    ///
    /// # References
    ///
    /// - [rfc5988#ref-W3C.REC-html401-19991224](https://tools.ietf.org/html/rfc5988#ref-W3C.REC-html401-19991224)
    Subsection,

    /// Points to a resource containing the successor version
    /// in the version history.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    SuccessorVersion,

    /// Refers to a parent document in a hierarchy of
    /// documents.
    ///
    /// # Notes
    ///
    /// this relation type registration did not indicate a
    /// reference.  Requested by Noah Slater.
    ///
    /// # References
    ///
    /// - [RFC5988](https://tools.ietf.org/html/RFC5988)
    Up,

    /// Points to a resource containing the version history
    /// for the context.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    VersionHistory,

    /// Identifies a resource that is the source of the
    /// information in the link's context.
    ///
    /// # References
    ///
    /// - [RFC4287](https://tools.ietf.org/html/RFC4287)
    Via,

    /// Points to a working copy for this resource.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    WorkingCopy,

    /// Points to the versioned resource from which this
    /// working copy was obtained.
    ///
    /// # References
    ///
    /// - [RFC5829](https://tools.ietf.org/html/RFC5829)
    WorkingCopyOf,
}

impl Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alternate => write!(f, "alternate"),
            Self::Appendix => write!(f, "appendix"),
            Self::Bookmark => write!(f, "bookmark"),
            Self::Chapter => write!(f, "chapter"),
            Self::Contents => write!(f, "contents"),
            Self::Copyright => write!(f, "copyright"),
            Self::Current => write!(f, "current"),
            Self::Describedby => write!(f, "describedby"),
            Self::Edit => write!(f, "edit"),
            Self::EditMedia => write!(f, "edit-media"),
            Self::Enclosure => write!(f, "enclosure"),
            Self::First => write!(f, "first"),
            Self::Glossary => write!(f, "glossary"),
            Self::Help => write!(f, "help"),
            Self::Hub => write!(f, "hub"),
            Self::Index => write!(f, "index"),
            Self::Last => write!(f, "last"),
            Self::LatestVersion => write!(f, "latest-version"),
            Self::License => write!(f, "license"),
            Self::Next => write!(f, "next"),
            Self::NextArchive => write!(f, "next-archive"),
            Self::Payment => write!(f, "payment"),
            Self::Prev => write!(f, "prev"),
            Self::PredecessorVersion => write!(f, "predecessor-version"),
            Self::Previous => write!(f, "previous"),
            Self::PrevArchive => write!(f, "prev-archive"),
            Self::Related => write!(f, "related"),
            Self::Replies => write!(f, "replies"),
            Self::Section => write!(f, "section"),
            Self::Self_ => write!(f, "self"),
            Self::Service => write!(f, "service"),
            Self::Start => write!(f, "start"),
            Self::Stylesheet => write!(f, "stylesheet"),
            Self::Subsection => write!(f, "subsection"),
            Self::SuccessorVersion => write!(f, "successor-version"),
            Self::Up => write!(f, "up"),
            Self::VersionHistory => write!(f, "version-history"),
            Self::Via => write!(f, "via"),
            Self::WorkingCopy => write!(f, "working-copy"),
            Self::WorkingCopyOf => write!(f, "working-copy-of"),
        }
    }
}
impl FromStr for RelationType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alternate" => Ok(Self::Alternate),
            "appendix" => Ok(Self::Appendix),
            "bookmark" => Ok(Self::Bookmark),
            "chapter" => Ok(Self::Chapter),
            "contents" => Ok(Self::Contents),
            "copyright" => Ok(Self::Copyright),
            "current" => Ok(Self::Current),
            "describedby" => Ok(Self::Describedby),
            "edit" => Ok(Self::Edit),
            "edit-media" => Ok(Self::EditMedia),
            "enclosure" => Ok(Self::Enclosure),
            "first" => Ok(Self::First),
            "glossary" => Ok(Self::Glossary),
            "help" => Ok(Self::Help),
            "hub" => Ok(Self::Hub),
            "index" => Ok(Self::Index),
            "last" => Ok(Self::Last),
            "latest-version" => Ok(Self::LatestVersion),
            "license" => Ok(Self::License),
            "next" => Ok(Self::Next),
            "next-archive" => Ok(Self::NextArchive),
            "payment" => Ok(Self::Payment),
            "prev" => Ok(Self::Prev),
            "predecessor-version" => Ok(Self::PredecessorVersion),
            "previous" => Ok(Self::Previous),
            "prev-archive" => Ok(Self::PrevArchive),
            "related" => Ok(Self::Related),
            "replies" => Ok(Self::Replies),
            "section" => Ok(Self::Section),
            "self" => Ok(Self::Self_),
            "service" => Ok(Self::Service),
            "start" => Ok(Self::Start),
            "stylesheet" => Ok(Self::Stylesheet),
            "subsection" => Ok(Self::Subsection),
            "successor-version" => Ok(Self::SuccessorVersion),
            "up" => Ok(Self::Up),
            "version-history" => Ok(Self::VersionHistory),
            "via" => Ok(Self::Via),
            "working-copy" => Ok(Self::WorkingCopy),
            "working-copy-of" => Ok(Self::WorkingCopyOf),
            s => bail!(400, "{} is not a recognized relation type", s),
        }
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7865c2b552623ee5a770c377664bcedd
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d70ff41ea9fcfa8a94a1584f48a464e3
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e58695d3302e5086224847133c539344
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e58695d3302e5086224847133c539344
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e58695d3302e5086224847133c539344
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e58695d3302e5086224847133c539344
