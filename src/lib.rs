/**
### This enum is used to represent the type of link layer found in a pcap file.
Each variant represents a different type of link layer, with a corresponding integer value that is present in the last 4 bytes of a pcap file.

If you find a mistake or want to add a new variant, please open an issue or a pull request.

Reference: https://www.tcpdump.org/linktypes.html

#### Example
```rust
use linktype::LinkType;

fn main() {
    let link_type = LinkType::Ethernet;
    println!("Link type: {:?}", link_type);

    match link_type {
        LinkType::Ethernet => {
            // Adjust code execution according to the link type
        },
        _ => println!("Unknown link type"),
    }
}
```

 */
#[derive(Debug)]
pub enum LinkType {
    Null = 0,
    Ethernet = 1,
    ExpEthernet = 2,
    AX25 = 3,
    Pronet = 4,
    Chaos = 5,
    Ieee8025 = 6,
    ArcnetBSD = 7,
    Slip = 8,
    Ppp = 9,
    Fddi = 10,
    PppHdlc = 50,
    PppEther = 51,
    SymantecFirewall = 99,
    AtmRfc1483 = 100,
    Raw = 101,
    Chdlc = 104,
    Ieee80211 = 105,
    AtmClip = 106,
    Frelay = 107,
    Loop = 108,
    Enc = 109,
    NetBSDHdlc = 112,
    LinuxSll = 113,
    Ltalk = 114,
    Pflog = 117,
    Ieee80211Prism = 119,
    IpOverFc = 122,
    Sunatm = 123,
    Ieee80211Radiotap = 127,
    Tzsp = 128,
    ArcnetLinux = 129,
    JuniperMlppp = 130,
    JuniperMlfr = 131,
    JuniperEs = 132,
    JuniperGgsn = 133,
    JuniperMfr = 134,
    JuniperAtm2 = 135,
    JuniperServices = 136,
    JuniperAtm1 = 137,
    AppleIpOverIeee1394 = 138,
    Mtp2WithPhdr = 139,
    Mtp2 = 140,
    Mtp3 = 141,
    Sccp = 142,
    Docsis = 143,
    LinuxIrda = 144,
    IbmSp = 145,
    IbmSn = 146,
    Ieee80211Avs = 163,
    JuniperMonitor = 164,
    BacnetMsTp = 165,
    PppPppd = 166,
    JuniperPpoe = 167,
    JuniperPpoeAtm = 168,
    GprsLlc = 169,
    GpfT = 170,
    GpfF = 171,
    GcomT1E1 = 172,
    GcomSerial = 173,
    JuniperPicPeer = 174,
    ErfEth = 175,
    ErfPos = 176,
    LinuxLapd = 177,
    JuniperEther = 178,
    JuniperPpp = 179,
    JuniperFrelay = 180,
    JuniperChdlc = 181,
    Mfr = 182,
    JuniperVp = 183,
    A429 = 184,
    A653Icm = 185,
    UsbFreebsd = 186,
    BluetoothHciH4 = 187,
    Ieee80216MacCps = 188,
    UsbLinux = 189,
    Can20B = 190,
    Ieee802154Linux = 191,
    Ppi = 192,
    Ieee80216MacCpsRadio = 193,
    JuniperIsm = 194,
    Ieee802154WithFcs = 195,
    Sita = 196,
    Erf = 197,
    Raif1 = 198,
    IpmbKontron = 199,
    JuniperSt = 200,
    BluetoothHciH4WithPhdr = 201,
    Ax25Kiss = 202,
    Lapd = 203,
    PppWithDir = 204,
    CHdlcWithDir = 205,
    FrelayWithDir = 206,
    LapbWithDir = 207,
    I2CLinux = 209,
    Most = 211,
    Lin = 212,
    X2eSerial = 213,
    X2eXoraya = 214,
    Ieee802154NonaskPhy = 215,
    LinuxEvdev = 216,
    GsmtapUm = 217,
    GsmtapAbis = 218,
    Mpls = 219,
    UsbLinuxMmapped = 220,
    Dect = 221,
    Aos = 222,
    Wihart = 223,
    Fc2 = 224,
    Fc2WithFrameDelims = 225,
    Ipnet = 226,
    CanSocketcan = 227,
    Ipv4 = 228,
    Ipv6 = 229,
    Ieee802154Nofcs = 230,
    Dbus = 231,
    JuniperVs = 232,
    JuniperSrxE2e = 233,
    JuniperFibreChannel = 234,
    DvbCi = 235,
    Mux27010 = 236,
    Stanag5066DPdu = 237,
    JuniperAtmCemic = 238,
    Nflog = 239,
    Netanalyzer = 240,
    NetanalyzerTransparent = 241,
    Ipoib = 242,
    Mpeg2Ts = 243,
    Ng40 = 244,
    NfcLlcp = 245,
    PfSync = 246,
    Infiniband = 247,
    Sctp = 248,
    Usbpcap = 249,
    RtacSerial = 250,
    BluetoothLELL = 251,
    WiresharkUpperPdu = 252,
    Netlink = 253,
    BluetoothLinuxMonitor = 254,
    BluetoothBredrBb = 255,
    BluetoothLELLWithPhdr = 256,
    ProfibusDl = 257,
    Pktap = 258,
    Epon = 259,
    IpmiHpm2 = 260,
    ZwaveR1R2 = 261,
    ZwaveR3 = 262,
    WattstopperDlm = 263,
    Iso14443 = 264,
    Rds = 265,
    UsbDarwin = 266,
    Openflow = 267,
    Sdlc = 268,
    TILlnSniffer = 269,
    Loratap = 270,
    Vsock = 271,
    NordicBLE = 272,
    Docsis31Xra31 = 273,
    EthernetMpacket = 274,
    DisplayportAux = 275,
    LinuxSll2 = 276,
    SercosMonitor = 277,
    OpenVizsla = 278,
    Ebhscr = 279,
    VppDispatch = 280,
    DsaTagBrcm = 281,
    DsaTagBrcmPrepend = 282,
    IEEE802154Tap = 283,
    DsaTagDsa = 284,
    DsaTagEdsa = 285,
    Elee = 286,
    ZWaveSerial = 287,
    Usb20 = 288,
    AtscAlp = 289,
    Etw = 290,
    NetanalyzerNg = 291,
    ZbossNcp = 292,
    Usb20LowSpeed = 293,
    Usb20FullSpeed = 294,
    Usb20HighSpeed = 295,
    AuerswaldLog = 296,
    ZwaveTap = 297,
    SiLabsDebugChannel = 298,
    FiraUCI = 299,
    Mdbus = 300,
    DectNR = 301,
}
