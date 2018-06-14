extern crate lazyf;
extern crate mksvg;

use lazyf::lzlist::LzList;
use lazyf::cfg::Cfg;
use lazyf::get::SGetter;

mod mcard;
mod cdenv;

use mcard::MCard;


use mksvg::page;

use std::io::stdout;



fn main() {
    let cf = Cfg::load_first("-c",&["{HOME}/.config/monchters/init.lz","conf.lz"]);

    let clocs = cf.get_s_def(("-cds","cards"),"cards.lz");


    let mut cit = clocs.split(',').map(|s|cf.localize(s));


    let cardlz = LzList::load_all(&mut cit);
    let cde = cdenv::CDEnv::from_cfg(&cf);

    let cards:Vec<MCard> = cardlz.iter().map(|lz|MCard::from_lz_env(lz,&cde)).collect();

    page::page_a4(stdout(),5,7,&cards);

}
