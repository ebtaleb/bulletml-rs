#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xml;
extern crate bulletml;

use std::fs::File;
use std::collections::HashMap;

use xml::reader::EventReader;
use xml::reader::events::*;

use bulletml::*;

fn indent(size: usize) -> String {
    let indent: &'static str = "    ";
    (0..size).map(|_| indent)
             .fold(String::with_capacity(size*indent.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
    info!("starting up");

    let file = File::open("src/sample_2.xml").unwrap();
    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut curr_tag = "".to_string();
    let mut parent_tag = "".to_string();

    let mut parent_label = "".to_string();
    let mut curr_label = "".to_string();

    let mut bulletml_pattern : bulletml::BulletMLBody =
        bulletml::BulletMLBody {
            attribute : bulletml::BarrageType::NoBarrage,
            tofire : Vec::new(),
            bullets : Vec::new(),
            actions : Vec::new()
        };

    let mut muh_bullets = HashMap::new();
    let mut muh_fires = HashMap::new();
    let mut muh_actions = HashMap::new();

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {

                curr_tag = name.local_name.to_string();

                if parent_tag == "" {
                    parent_tag = name.local_name.to_string()
                } else {
                    parent_tag = curr_tag
                }

                println!("{}+{} {:?}", indent(depth), name.local_name, attributes);
                depth += 1;

                match name.local_name.as_ref() {
                    "bulletml" => {
                        match attributes.is_empty() {
                            true => {}
                            false =>  { bulletml_pattern.attribute = match attributes[0].value.as_ref() {
                                        "vertical" => bulletml::BarrageType::VerticalBarrage,
                                        "horizontal" => bulletml::BarrageType::HorizontalBarrage,
                                        _ => bulletml::BarrageType::NoBarrage
                                    };
                            }
                        }
                    }

                    "bullet" => {
                        match attributes.is_empty() {
                            true => {}
                            false =>  { curr_label = attributes[0].value.to_string();
                                        let label_name = attributes[0].value.to_string();
                                        let mut new_bullet = bulletml::Bullet::BulletDef {
                                            label : Some(label_name.to_string()),
                                            direction : None,
                                            speed : None,
                                            actions : Vec::new()
                                        };
                                        //bulletml_pattern.bullets.push(new_bullet);
                                        muh_bullets.insert(label_name, new_bullet); {}
                            }
                        }
                    }

                    "action" => {
                        match attributes.is_empty() {
                            true => {}
                            false =>  { curr_label = attributes[0].value.to_string();
                                        let label_name = attributes[0].value.to_string();
                                        let mut new_action = bulletml::Action::ActionDef {
                                            label : Some(label_name.to_string()),
                                            actions : Vec::new(),
                                            tofire : Vec::new(),
                                            contents : Vec::new()
                                        };
                                        //bulletml_pattern.actions.push(new_action);
                                        muh_actions.insert(label_name, new_action); {}
                            }
                        }
                    }

                    "fire" => {
                        match attributes.is_empty() {
                            true => {}
                            false =>  { curr_label = attributes[0].value.to_string();
                                        let label_name = attributes[0].value.to_string();
                                        let mut new_tofire = bulletml::Fire::FireDef {
                                            label : Some(label_name.to_string()),
                                            direction : None,
                                            speed : None,
                                            bullet : bulletml::Bullet::NoBullet
                                        };
                                        //bulletml_pattern.tofire.push(new_tofire);
                                        muh_fires.insert(label_name, new_tofire); {}
                            }
                        }
                    }

                    "changeDirection" => {}
                    "changeSpeed" => {}
                    "accel" => {}
                    "wait" => {}
                    "vanish" => {}

                    "repeat" => {}
                    "direction" => {}
                    "speed" => {}

                    "horizontal" => {}
                    "vertical" => {}

                    "term" => {}
                    "bulletRef" => {}
                    "actionRef" => {}
                    "fireRef" => {}

                    "param" => {}
                    _ => {}
                }
            }

            XmlEvent::EndElement { name } => {
                //if curr_tag == name.local_name {
                    //println!("building tag {}", name)
                //}
                depth -= 1;
                println!("{}-{}", indent(depth), &name.local_name);
                curr_tag = parent_tag.to_string();
            }

            XmlEvent::Characters(s) => {
                println!("{} {}", indent(depth), s);
            }

            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break;
            }

            _ => {}
        }
    }

    println!("{:?}", bulletml_pattern);

    println!("{:?}", muh_bullets);
    println!("{:?}", muh_fires);
    println!("{:?}", muh_actions);
}
