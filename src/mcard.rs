use lazyf::lzlist::Lz;
use lazyf::get::SGetter;
use mksvg::page::Card;
use mksvg::{Svg,style,st};
use std::path::{Path,PathBuf};

#[derive(Clone)]
pub struct MCard {
    name:String,
    path:PathBuf,
    fol:String,
    elem:String,
    count:i32,
    movement:String,
    special:String,
    cost:i32, 
    bgcol:string,
}

impl MCard {
    pub fn fromLZnP(lz:&Lz,p:&Path)->MCard{
        MCard{
            name:lz.name.clone(),
            path:PathBuf::from(p),
            fol:lz.get_s_def("Fol","creatures"),
            elem:lz.get_s_def("Type","Brawn"),
            count:lz.get_t_def("ext0",0),
            movement:lz.get_s_def("Move","NONE"),
            cost:lz.get_t_def("Cost",3),
            special:lz.get_s_def("Special",""),
            bgcol:"#ffbbbb".to_string(),
        }
    }
}

impl Card<f64> for MCard {
    fn front<S:Svg>(&self,s:&mut S, w:f64,h:f64){
        let mut cpath = self.path.clone(); 
        cpath.push(&self.fol);
        cpath.push([&(self.name.to_lowercase()) , ".svg"].join(""));
        

        s.rect(0.0,0.0,w,h,&style(&[&st("stroke-width",w/10.0),"stroke:black;fill:white;"])); 

        s.img(cpath.to_string_lossy().to_mut(),0.0,0.0,w,h);
    }
}


