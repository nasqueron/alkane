//  -------------------------------------------------------------
//  Alkane :: Runner :: Site
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  Description:    Represent the website metadata
//  -------------------------------------------------------------

pub struct Site {
    pub name: String,
    pub path: String,

    /// The build context, any metadata relevant to the build
    pub context: Option<String>,
}
