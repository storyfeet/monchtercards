use std::path::PathBuf;
use lazyf::cfg::Cfg;
use lazyf::lzlist::Lz;
use lazyf::get::SGetter;





pub struct CDEnv {
    root:PathBuf,
    pub elem_fol:String,
    pub mon_fol:String,
    tmap:Lz;
}


impl CDEnv{
    pub fn from_cfg(cf:&Cfg)->CDEnv {
        let rpath = cf.get_s_def(("-lp","config.link-path"),cf.folder().clone().to_string_lossy().to_mut());
        CDEnv {
            root:PathBuf::from(&rpath),
            elem_fol:cf.get_s_def(("-elem","config.elem-path"),"elems"),
            mon_fol:cf.get_s_def(("-mon","config.mon-path"),"creatures"),
            tmap :match cf.lz_by_name("Map"){
                Some(z)=>z,
                None=>Lz::new("Map"),
            },
        }
    }

    pub fn picloc(&self,fol:&str,cname:&str)->String{
        let mut base = self.root.clone();
        if fol != "".to_string() {
            base.push(fol);
        }
        base.push(&format!("{}.svg",cname));
        base.to_string_lossy().to_mut().to_string()
    }

    pub fn mapMove(s:&str){
        //TODO --Masive convert Long names to short ones.
    }
}

