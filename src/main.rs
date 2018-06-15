extern crate lazyf;
extern crate mksvg;

use lazyf::lzlist::LzList;
use lazyf::cfg::Cfg;
use lazyf::get::SGetter;

mod mcard;
mod cdenv;

use mcard::MCard;


use mksvg::page;

//use std::io::stdout;



fn main() {
    let cf = Cfg::load_first("-c",&["{HOME}/.config/monchters/init.lz","conf.lz"]);

    let clocs = cf.get_s_def(("-cds","cards"),"cards.lz");


    let mut cit = clocs.split(',').map(|s|cf.localize(s));


    let cardlz = LzList::load_all(&mut cit);
    let cde = cdenv::CDEnv::from_cfg(&cf);

    let cards:Vec<MCard> = cardlz.iter().map(|lz|MCard::from_lz_env(lz,&cde)).collect();

    let fout = cf.localize(&cf.get_s_def(("-fout","config.out-front"),"out/f"));
    let mut fpages = page::pages_a4(fout,5,7,&cards);

    let mut cbacks = page::page_flip(&cards,5);
    for n in &mut cbacks {
        n.bgcol = "#bbffbb".to_string();
    }
    let bout = cf.localize(&cf.get_s_def(("-bout","config.out-back"),"out/b"));
    let bpages=   page::pages_a4(bout,5,7,&cbacks);

    fpages.extend(bpages);
    page::unite_as_pdf(fpages,cf.localize(&cf.get_s_def(("-pdf","out-pdf"),"out/all")));
}
