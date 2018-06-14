use std::path::PathBuf;
use lazyf::cfg::Cfg;
use lazyf::lzlist::Lz;
use lazyf::get::SGetter;





pub struct CDEnv {
    root:PathBuf,
    pub elem_fol:String,
    pub mon_fol:String,
    tmap:Lz,
}

fn map_moves(s:&str,lz:&Lz)->String{
    //TODO --Masive convert Long names to short ones.
    let mut res = String::new();
    let mut tmp = String::new();

    for c in s.chars(){
        match c {
            '{'|'}'|'('|')'|'['|']'|','=>{
                if tmp.len()>0{
                    res.push_str(&lz.get_s_def(&tmp,&tmp));
                    tmp.clear();
                }
                res.push(c);
            }
            _=>{
                tmp.push(c);
            }
        }
    }
    res.push_str(&lz.get_s_def(&tmp,&tmp));
    res
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

    pub fn map_moves(&self,s:&str)->String{
        map_moves(s,&self.tmap)
    }
}


#[cfg(test)]
mod tests {
    use cdenv::map_moves;
    use lazyf::lzlist::Lz;
    #[test]
    fn test_mapping(){
        let mut lz = Lz::new("Map");
        lz.add_deet("Castle","Cs");
        lz.add_deet("Bishop","Bs");

        assert_eq!(map_moves("Castle(4)",&lz),"Cs(4)");
        assert_eq!(map_moves("[Bishop,Castle](4)Castle",&lz),"[Bs,Cs](4)Cs");
    }

}
