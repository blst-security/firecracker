use super::*;
use rand::Rng;
use rand::distributions::Alphanumeric;
use uuid::Uuid;
use std::collections::HashMap;


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]
pub enum GenMethod{
    FullyInformed,
    VSelf,
    RandomInformed,
    Random,
}
fn convert(bits: Vec<u8>) -> i16 {
    bits.iter()
        .fold(0, |result, &bit| {
            (result << 1) ^ bit as i16
        })
}
fn generate_number_in_range(bits:Vec<u8>,min:i16,max:i16)->i16{
    let conv = convert(bits);
    //conv div full range mult part range + min
    (((conv as f64)/(32767.0*2.0))*(max-min) as f64 +min as f64) as i16
}
fn gen_number(method:GenMethod,bits:Vec<u8>,param:NumDescriptor)->i16{
    match method{
        GenMethod::FullyInformed=>{
            match param{
                NumDescriptor::Range((s,e))=>{
                    generate_number_in_range(bits,s as i16,e as i16)
                },
                NumDescriptor::List(lst)=>{
                    lst[generate_number_in_range(bits,0,lst.len() as i16-1) as usize] as i16
                },
                NumDescriptor::Random=>{
                    convert(bits)
                }
            }
        },
        GenMethod::VSelf=>{
            convert(bits)
        },
        GenMethod::RandomInformed=>{
            let mut rng =rand::thread_rng();
            match param{
                NumDescriptor::Range((s,e))=>{
                    rng.gen_range(s..e) as i16
                },
                NumDescriptor::List(lst)=>{
                    lst[rng.gen_range(0..lst.len())] as i16
                },
                NumDescriptor::Random=>{
                    rng.gen_range(-32768..32767)
                }
            }
        },
        GenMethod::Random=>{
            let mut rng =rand::thread_rng();
            rng.gen_range(-32768..32767)
        },
    }
}
fn gen_string(method:GenMethod,bits:Vec<u8>,param:StringDescriptor)->String{
    match method{
        GenMethod::FullyInformed=>{
            match param{
                StringDescriptor::Uuid(_)=>{
                    //future - from values vec
                    //we currently only support v4 in the attacker
                        /*
                    match v{
                        1=> Uuid::new_v1(),
                        3=> Uuid::new_v3(),
                        4=> Uuid::new_v4(),
                        5=> Uuid::new_v5(),
                    }*/
                    Uuid::new_v4().to_string()
                }
                StringDescriptor::List(lst)=>{
                    lst[generate_number_in_range(bits,0,lst.len() as i16-1) as usize].clone()
                }
                StringDescriptor::Random=>{
                    let sum1:u8 = bits.iter().sum();
                    let str1:String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(sum1 as usize)
                        .map(char::from)
                        .collect();
                    str1
                }
                _=>{
                    let sum1:u8 = bits.iter().sum();
                    let str1:String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(sum1 as usize)
                        .map(char::from)
                        .collect();
                    str1
                }
            }
        },
        GenMethod::VSelf=>{
            //repetitive for now, will be changed in later versions
            match param{
                StringDescriptor::Uuid(_)=>{
                    //future - from values vec
                    Uuid::new_v4().to_string()
                }
                StringDescriptor::List(lst)=>{
                    lst[generate_number_in_range(bits,0,lst.len() as i16-1) as usize].clone()
                }
                _=>{
                    let sum1:u8 = bits.iter().sum();
                    let str1:String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(sum1 as usize)
                        .map(char::from)
                        .collect();
                    str1
                }
            }
        },
        GenMethod::RandomInformed=>{
            match param{
                StringDescriptor::List(lst)=>{
                    let mut rng = rand::thread_rng();
                    lst[rng.gen_range(0..lst.len())].clone()
                }
                _=>{
                    let sum1:u8 = bits.iter().sum();
                    let str1:String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(sum1 as usize)
                        .map(char::from)
                        .collect();
                    str1
                }
            }
        },
        GenMethod::Random=>{
            let mut rng = rand::thread_rng();
            let sum1:usize = rng.gen_range(0..24); 
            let str1:String = rng
                .sample_iter(&Alphanumeric)
                .take(sum1)
                .map(char::from)
                .collect();
            str1
        }
    }
}
pub fn gen_type(bits:Vec<u8>)->GenMethod{
    let mut b = bits.chunks(bits.len()/2);
    let bits_first = b.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
    let bits_second = b.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
    if convert(bits_first.clone())>=convert(bits_second.clone()){
        let mut bb = bits_first.chunks(bits_first.len()/2);
        let b_f = bb.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
        let b_s = bb.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
        if convert(b_f)>=convert(b_s){
            GenMethod::FullyInformed
        }else{
            GenMethod::VSelf
        }
    }else{
        let mut bb = bits_second.chunks(bits_first.len()/2);
        let b_f = bb.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
        let b_s = bb.next().unwrap().iter().map(|n| *n).collect::<Vec<u8>>();
        if convert(b_f)>=convert(b_s){
            GenMethod::RandomInformed
        }else{
            GenMethod::Random
        }
    }
}
#[derive(Debug,Clone,Serialize,Deserialize)]
struct Parameter{
    name:String,
    value:String,
    #[serde(skip_serializing)]
    dm:QuePay,
}
async fn send_attack(base_url:&str,eps:Vec<(Method,String,Vec<Parameter>)>)->Vec<ReqRes>{
    let mut rr = vec![];
    let client = reqwest::Client::new();
    for ep in eps{
        match ep.0{
            Method::GET=>{
                let mut query = String::from("?");
                for param in ep.2{
                    if let QuePay::Query=param.dm{
                        query.push_str(&format!("{}{}",param.name,param.value));
                    }
                }
                query.pop();
                let res = reqwest::get(&format!("{}{}{}",base_url,ep.1.clone(),query)).await.unwrap();
                rr.push(ReqRes{
                    req_headers:HashMap::new(),
                    res_headers:HashMap::new(),
                    path:ep.1,
                    method:Method::GET,
                    status:res.status().as_u16(),
                    req_payload:String::new(),
                    res_payload:res.text().await.unwrap(),
                    req_query:query,
                });
            },
            Method::POST=>{
                let payload = serde_json::to_string(&ep.2).unwrap();
                let res = client.post(&format!("{}{}",base_url,ep.1.clone()))
                    .body(payload.clone())
                    .send()
                    .await.unwrap();
                rr.push(ReqRes{
                    req_headers:HashMap::new(),
                    res_headers:HashMap::new(),
                    path:ep.1,
                    method:Method::POST,
                    status:res.status().as_u16(),
                    req_payload:payload,
                    res_payload:res.text().await.unwrap(),
                    req_query:String::new(),
                });
            },
            Method::PATCH=>{
                let payload = serde_json::to_string(&ep.2).unwrap();
                let res = client.patch(&format!("{}{}",base_url,ep.1.clone()))
                    .body(payload.clone())
                    .send()
                    .await.unwrap();
                rr.push(ReqRes{
                    req_headers:HashMap::new(),
                    res_headers:HashMap::new(),
                    path:ep.1,
                    method:Method::POST,
                    status:res.status().as_u16(),
                    req_payload:payload,
                    res_payload:res.text().await.unwrap(),
                    req_query:String::new(),
                });
            },
            _=>{}
        }

    }
    rr
}
pub async fn attack_flow(base_url:&str,genes:&Vec<Gene>)->(Vec<ReqRes>,Vec<String>){
    let mut eps = vec![];
    let mut choises =vec![];
    for gene in genes{
        let mut params:Vec<Parameter> = vec![];
        for c in gene.chromosomes(){
            let value = match &c.descriptor{
                ValueDescriptor::Number((nd,_)) => {
                    let choise =gen_type(c.dna.clone());
                    choises.push(format!("{:?}",choise.clone()));
                    gen_number(choise,c.dna.clone(),nd.clone()).to_string()
                },
                ValueDescriptor::String(d)=>{
                    let choise =gen_type(c.dna.clone());
                    choises.push(format!("{:?}",choise.clone()));
                    gen_string(choise,c.dna.clone(),d.clone())
                },
                ValueDescriptor::Bool=>{
                    if c.dna[0]==1{
                        choises.push(String::from("bool:true"));
                        String::from("true")
                    }else{
                        choises.push(String::from("bool:true"));
                        String::from("false")
                    }
                },
                ValueDescriptor::Unknown=> {
                    let choise =gen_type(c.dna.clone());
                    choises.push(format!("{:?}",choise.clone()));
                    gen_string(choise,c.dna.clone(),StringDescriptor::Random)
                }
            };
            params.push(Parameter{
               name:c.param_name.clone(),
               value,
               dm:c.delivery_method.clone(),
            });           
        }
        eps.push((gene.method,gene.ep.clone(),params));
    }
    (send_attack(base_url,eps).await,choises)
}
