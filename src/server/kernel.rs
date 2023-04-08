//  -------------------------------------------------------------
//  Alkane :: Server :: Kernel
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
//  Project:        Nasqueron
//  License:        BSD-2-Clause
//  -------------------------------------------------------------

use rocket::ignite;
use rocket_codegen::routes;

use crate::config::AlkaneConfig;
use crate::server::requests::*;

//  -------------------------------------------------------------
//  Server entry point
//  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn run(config: AlkaneConfig, mounting_point: &str) {
    let routes = routes![
        // Monitoring
        status,
        // Alkane API
        init,
        update,
        deploy,
        is_present,
    ];

    ignite()
        .manage(config)
        .mount(mounting_point, routes)
        .launch();
}
