use lazyf::lzlist::Lz;
use lazyf::get::SGetter;
use mksvg::page::Card;
use mksvg::{Svg,style,st};
//use std::path::{Path,PathBuf};
use cdenv::CDEnv;

#[derive(Clone)]
pub struct MCard<'a> {
    name:String,
    env:&'a CDEnv,
    fol:String,
    elem:String,
    count:i32,
    movement:String,
    special:String,
    cost:i32, 
    bgcol:String,
}

impl<'a> MCard<'a> {
    pub  fn  from_lz_env(lz:&Lz,e:&'a CDEnv)->MCard<'a>{
        MCard{
            name:lz.name.clone(),
            env:e,
            fol:lz.get_s_def("Fol",&e.mon_fol),
            elem:lz.get_s_def("Type","Brawn"),
            count:lz.get_t_def("ext0",0),
            movement:lz.get_s_def("Move","NONE"),
            cost:lz.get_t_def("Cost",3),
            special:lz.get_s_def("Special",""),
            bgcol:"#ffbbbb".to_string(),
        }
    }
}

impl<'a> Card<f64> for MCard<'a> {
    fn front<S:Svg>(&self,s:&mut S, w:f64,h:f64){

        s.rect(0.0,0.0,w,h,&style(&[&st("stroke-width",w/30.0),&st("fill",&self.bgcol),"stroke:black;"])); 

        let cpath = self.env.picloc(&self.fol,&self.name.to_lowercase());
        s.img(&cpath,w*0.15,w*0.1,w*0.7,w*0.7);

        let epath = self.env.picloc(&self.env.elem_fol,&self.elem.to_lowercase());
        s.img(&epath,5.0,5.0,w/5.0,w/5.0);

        //cost
        if self.cost != 0 {
            s.ellipse(w*0.9,h*0.1,w*0.06,w*0.06,&style(&[&st("stroke-width",w*0.01),"fill:yellow;stroke:black;"]));
            s.text(&self.cost.to_string(),w*0.87,h*0.13,w*0.1,"",&["text:anchor:middle;stroke:none;fill:black;font-weight:bold"]);
        }

        //Text and abilities
        s.text(&self.name,w/2.0,h*0.85,w*0.09,"",&["text-anchor:middle;stroke:none;fill:black;"]);
        s.text(&self.env.map_moves(&self.movement),w*0.015,h*0.94,w*0.075,"",&["text-anchor:start;stroke:none;fill:black"]);
        s.text(&self.env.map_moves(&self.special),w*0.985,h*0.94,w*0.075,"",&["text-anchor:end;stroke:none;fill:black"]);
    }
}


