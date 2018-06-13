extern crate lazyf;
extern crate mksvg;

use lazyf::lzlist::LzList;
use lazyf::cfg::Cfg;
use lazyf::get::SGetter;

mod mcard;

use mcard::MCard;


use mksvg::page;

use std::io::stdout;



fn main() {
    let cf = Cfg::load_first("-c",&["{HOME}/.config/monchters/init.lz","conf.lz"]);
    let clocs = cf.get_s_def(("-cds","cards"),"cards.lz");
    let p= cf.folder();

    let mut cit = clocs.split(',').map(|s|cf.localize(s));


    let cardlz = LzList::load_all(&mut cit);

    let cards:Vec<MCard> = cardlz.iter().map(|lz|MCard::fromLZnP(lz,&p)).collect();

    page::page_a4(stdout(),5,7,&cards);

}
