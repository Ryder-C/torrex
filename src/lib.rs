pub mod bencode;

#[cfg(test)]
mod tests {
    use super::*;
    use bencode::{File, Torrent};
    use chrono::DateTime;
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_ubuntu_torrent_decoding() {
        // Test the decoding of a basic torrent file
        let torrent_path = "test_data/ubuntu.torrent";
        let torrent = Torrent::new(torrent_path);
        assert!(torrent.is_ok(), "Failed to decode the torrent file");

        let torrent = torrent.unwrap();
        assert_eq!(
            torrent.info.name,
            "ubuntu-24.04.1-desktop-amd64.iso".to_owned(),
            "Failed to decode name"
        );
        assert_eq!(
            torrent.info.files,
            vec![File {
                length: 6203355136,
                md5sum: None,
                path: PathBuf::from(""),
            }],
            "Failed to decode files"
        );
        assert_eq!(
            torrent.info.hash,
            [74, 63, 94, 8, 188, 239, 130, 87, 24, 237, 163, 6, 55, 35, 5, 133, 227, 51, 5, 153,],
            "Failed to compute correct hash"
        );
        assert_eq!(
            torrent.announce_list,
            vec![
                "https://ipv6.torrent.ubuntu.com/announce".to_owned(),
                "https://torrent.ubuntu.com/announce".to_owned()
            ],
            "Failed to decode announce urls"
        );
        assert_eq!(
            torrent.creation_date,
            Some(DateTime::from_str("2024-08-29T09:03:35-07:00").unwrap()),
            "Failed to decode creation date"
        );
        assert_eq!(
            torrent.comment,
            Some("Ubuntu CD releases.ubuntu.com".to_owned()),
            "Failed to decode comment"
        );
        assert_eq!(
            torrent.created_by,
            Some("mktorrent 1.1".to_owned()),
            "Failed to decode created by"
        );
        assert!(torrent.encoding.is_none(), "Failed to decode encoding");
    }
}
