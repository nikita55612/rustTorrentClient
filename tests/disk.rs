// use rutor::{disk, torrent};

// #[test]
// fn test_disk_tree_layout() {
//     let mut dl = disk::layout::TreeLayout::new("tt");
//     dl.add_file("ggg.rs", 334);
//     dl.add_file("music/top/1.mp3", 455);
//     dl.add_file("music/top/w.mp3", 455);
//     dl.add_file("music/top/3.mp3", 455);
//     dl.add_file("music/top/4.mp3", 455);
//     dl.add_file("music/top/5.mp3", 455);

//     dl.add_file("music/top33/33/5.mp3", 455);
//     dl.add_file("music/top44/44/5.mp3", 455);
//     dl.add_file("music/top44/44/5.mp3", 455);
//     dl.add_file("music/top3/5.mp3", 455);

//     println!("{:#?}", dl);
// }

// #[test]
// fn test_disk_layout() {
//     let bytes = std::fs::read("resources/Red_Hot_Chili_Peppers.torrent").unwrap();
//     let metainfo = torrent::MetaInfo::from_bytes(&bytes).unwrap();

//     let dl = disk::layout::Layout::from_torrent_info(&metainfo.info);

//     println!("{:#?}", dl);
// }
