extern crate glui;
extern crate rusty;
extern crate gl;

use std::collections::HashMap;
use rusty::*;
use mecs::*;
use tools::*;

use tools::gltraits::gl_get_string;

fn main() {
    #[allow(unused_variables)]
    let w: World = match World::new_winless(Vec3::grey(0.1)) {
        Ok(w) => w,
        Err(e) => {
            println!("Failed to create an OpenGL context, your platform might not support OpenGL!\n\n");
            println!("The creation function returned error: {:?}", e);
            return;
        }
    };
    
    let ver = gl_get_string(gl::VERSION);
    
    println!("OpenGL Version: {}", ver);
    println!("OpenGL Vendor: {}", gl_get_string(gl::VENDOR));
    println!(
        "OpenGLSL Version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );
    
    let major = ver.as_bytes()[0] - '0' as u8;
    let minor = ver.as_bytes()[2] - '0' as u8;
    // println!("{:?}",(major, minor));
    
    let mut version_to_year = HashMap::new();
    version_to_year.insert((4,6),2017);
    version_to_year.insert((4,5),2014);
    version_to_year.insert((4,4),2013);
    version_to_year.insert((4,3),2012);
    version_to_year.insert((4,2),2011);
    version_to_year.insert((4,1),2010);
    version_to_year.insert((4,0),2010);
    version_to_year.insert((3,3),2010);
    version_to_year.insert((3,2),2009);
    version_to_year.insert((3,1),2009);
    version_to_year.insert((3,0),2008);
    version_to_year.insert((2,1),2006);
    version_to_year.insert((2,0),2004);
    version_to_year.insert((1,5),2003);
    version_to_year.insert((1,4),2002);
    version_to_year.insert((1,3),2001);
    version_to_year.insert((1,2),1998);
    version_to_year.insert((1,1),1997);
    version_to_year.insert((1,0),1992);
    
    if let Some(year) = version_to_year.get(&(major,minor)) {
        println!("OpenGL {}.{} was released in {}", major, minor, year);
    }
}
