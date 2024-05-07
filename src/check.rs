use std::collections::HashMap;
pub fn get<T: AsRef<str>>(get: T) -> Option<(String, String, u16)> {
    let mut HashMapDocumentations: HashMap<String, (String, String, u16)> = HashMap::new();
    let insert0 = (
        String::from_utf8_lossy(&[
            0x6f, 0x75, 0x74, 0x6c, 0x6f, 0x6f, 0x6b, 0x2e, 0x63, 0x6f, 0x6d,
        ])
        .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x2e, 0x6f, 0x66, 0x66, 0x69, 0x63, 0x65, 0x33, 0x36, 0x35,
            0x2e, 0x63, 0x6f, 0x6d,
        ])
        .to_string(),
        0x24b,
    );
    let insert1 = (
        String::from_utf8_lossy(&[
            0x68, 0x6f, 0x74, 0x6d, 0x61, 0x69, 0x6c, 0x2e, 0x63, 0x6f, 0x6d,
        ])
        .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x2e, 0x6f, 0x66, 0x66, 0x69, 0x63, 0x65, 0x33, 0x36, 0x35,
            0x2e, 0x63, 0x6f, 0x6d,
        ])
        .to_string(),
        0x24b,
    );
    let insert2 = (
        String::from_utf8_lossy(&[0x75, 0x6f, 0x6c, 0x2e, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x72])
            .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x73, 0x2e, 0x75, 0x6f, 0x6c, 0x2e, 0x63, 0x6f, 0x6d, 0x2e,
            0x62, 0x72,
        ])
        .to_string(),
        0x24b,
    );
    let insert3 = (
        String::from_utf8_lossy(&[0x62, 0x6f, 0x6c, 0x2e, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x72])
            .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x73, 0x2e, 0x62, 0x6f, 0x6c, 0x2e, 0x63, 0x6f, 0x6d, 0x2e,
            0x62, 0x72,
        ])
        .to_string(),
        0x24b,
    );
    let insert4 = (
        String::from_utf8_lossy(&[0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x72])
            .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x2e, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x72,
        ])
        .to_string(),
        0x24b,
    );
    let insert5 = (
        String::from_utf8_lossy(&[
            0x74, 0x65, 0x72, 0x72, 0x61, 0x2e, 0x63, 0x6f, 0x6d, 0x2e, 0x62, 0x72,
        ])
        .to_string(),
        String::from_utf8_lossy(&[
            0x73, 0x6d, 0x74, 0x70, 0x2e, 0x74, 0x65, 0x72, 0x72, 0x61, 0x2e, 0x63, 0x6f, 0x6d,
            0x2e, 0x62, 0x72,
        ])
        .to_string(),
        0x24b,
    );
    HashMapDocumentations.insert(insert0.0.clone(), insert0);
    HashMapDocumentations.insert(insert1.0.clone(), insert1);
    HashMapDocumentations.insert(insert2.0.clone(), insert2);
    HashMapDocumentations.insert(insert3.0.clone(), insert3);
    HashMapDocumentations.insert(insert4.0.clone(), insert4);
    HashMapDocumentations.insert(insert5.0.clone(), insert5);

    if let Some(value) = HashMapDocumentations.get(get.as_ref()) {
        let value = value.to_owned();
        Some(value)
    } else {
        None
    }
}
#[derive(Debug, Clone)]
pub struct TypesProtocol {
    pub host: String,
    pub port: i32,
    pub filename: String,
}

pub fn ViewimapSmtp(
    get: Option<i32>,
) -> Result<HashMap<i32, TypesProtocol>, Option<TypesProtocol>> {
    let mut Protocols: HashMap<i32, TypesProtocol> = HashMap::new();
    Protocols.insert(
        1,
        TypesProtocol {
            host: "imap.hushmail.com".to_string(),
            port: 993,
            filename: "husmail".to_string(),
        },
    );
    Protocols.insert(
        2,
        TypesProtocol {
            host: "imap.vivaldi.net".to_string(),
            port: 993,
            filename: "vivaldi".to_string(),
        },
    );
     Protocols.insert(
        30,
        TypesProtocol {
            host: "recibir.arquired.es".to_string(),
            port: 110,
            filename: "arquired_es".to_string(),
        },
    );

    Protocols.insert(
        3,
        TypesProtocol {
            host: "imap.strato.com".to_string(),
            port: 993,
            filename: "strato".to_string(),
        },
    );

    Protocols.insert(
        4,
        TypesProtocol {
            host: "imap.hostinger.com".to_string(),
            port: 993,
            filename: "hostinger".to_string(),
        },
    );

    Protocols.insert(
        5,
        TypesProtocol {
            host: "outlook.office365.com".to_string(),
            port: 993,
            filename: "outlook".to_string(),
        },
    );

    Protocols.insert(
        6,
        TypesProtocol {
            host: "outlook.office365.com".to_string(),
            port: 993,
            filename: "hotmail".to_string(),
        },
    );
    Protocols.insert(
        7,
        TypesProtocol {
            host: "imap.uhserver.com".to_string(),
            port: 993,
            filename: "uol".to_string(),
        },
    );
    Protocols.insert(
        8,
        TypesProtocol {
            host: "imap.bol.com.br".to_string(),
            port: 993,
            filename: "bol".to_string(),
        },
    );
    Protocols.insert(
        9,
        TypesProtocol {
            host: "imap.ig.com.br".to_string(),
            port: 995,
            filename: "ig".to_string(),
        },
    );
    Protocols.insert(
        10,
        TypesProtocol {
            host: "imap.superig.com.br".to_string(),
            port: 995,
            filename: "superIg".to_string(),
        },
    );
    Protocols.insert(
        11,
        TypesProtocol {
            host: "imap.terra.com.br".to_string(),
            port: 993,
            filename: "terra".to_string(),
        },
    );
    Protocols.insert(
        12,
        TypesProtocol {
            host: "imap.mail.yahoo.com".to_string(),
            port: 993,
            filename: "Yahoo".to_string(),
        },
    );
    Protocols.insert(
        13,
        TypesProtocol {
            host: "imap.aol.com".to_string(),
            port: 993,
            filename: "aol".to_string(),
        },
    );
    Protocols.insert(
        14,
        TypesProtocol {
            host: "imap.yandex.com".to_string(),
            port: 993,
            filename: "Yandex".to_string(),
        },
    );
    Protocols.insert(
        15,
        TypesProtocol {
            host: "imap.gmx.com".to_string(),
            port: 993,
            filename: "gmx".to_string(),
        },
    );
    Protocols.insert(
        16,
        TypesProtocol {
            host: "imap.fastmail.com".to_string(),
            port: 993,
            filename: "fastmail".to_string(),
        },
    );
    Protocols.insert(
        17,
        TypesProtocol {
            host: "imap.mail.att.net".to_string(),
            port: 993,
            filename: "att".to_string(),
        },
    );
    Protocols.insert(
        18,
        TypesProtocol {
            host: "imap.gmx.net".to_string(),
            port: 993,
            filename: "Gmx_de".to_string(),
        },
    );
Protocols.insert(
        19,
        TypesProtocol {
            host: "imap.orange.fr".to_string(),
            port: 993,
            filename: "Orange_fr".to_string(),
        },
    );
Protocols.insert(
        20,
        TypesProtocol {
            host: "imap-mail.outlook.com".to_string(),
            port: 993,
            filename: "live_fr".to_string(),
        },
    );
Protocols.insert(
        21,
        TypesProtocol {
            host: "imap.free.fr".to_string(),
            port: 993,
            filename: "free_fr".to_string(),
        },
    );
Protocols.insert(
        22,
        TypesProtocol {
            host: "imap.gmx.net".to_string(),
            port: 993,
            filename: "Gmx_de".to_string(),
        },
    );
Protocols.insert(
        23,
        TypesProtocol {
            host: "imap.orange.fr".to_string(),
            port: 993,
            filename: "wanadoo_fr".to_string(),
        },
    );
    Protocols.insert(
        24,
        TypesProtocol {
            host: "imap.ionos.es".to_string(),
            port: 993,
            filename: "ionos_es".to_string(),
        },
    );
    Protocols.insert(
        25,
        TypesProtocol {
            host: "imap.ionos.com".to_string(),
            port: 993,
            filename: "ionos".to_string(),
        },
    );
     Protocols.insert(
        26,
        TypesProtocol {
            host: "smtp.office365.com".to_string(),
            port: 587,
            filename: "Smpt".to_string(),
        },
    );

    Protocols.insert(
        27,
        TypesProtocol {
            host: "imap.mail.co.uk".to_string(),
            port: 993,
            filename: "mail".to_string(),
        },
    );
  Protocols.insert(
        30,
        TypesProtocol {
            host: "buzon.uma.es".to_string(),
            port: 993,
            filename: "Uma_es".to_string(),
        },
    );

	 Protocols.insert(
        45,
        TypesProtocol {
            host: "imap.uol.com.br".to_string(),
            port: 993,
            filename: "uolimapconta".to_string(),
        },
    );
    Protocols.insert(
        100,
        TypesProtocol {
            host: "Bypass".to_string(),
            port: 993,
            filename: "Bypass".to_string(),
        },
    );
    match get {
        Some(get) => {
            if let Some(index) = Protocols.get(&get) {
                return Err(Some(index.clone()));
            }
            return Err(None);
        }
        _ => {
            return Ok(Protocols);
        }
    }
}
