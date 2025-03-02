/// Layers on which an error can occur.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Layer {
    /// Error occured in the ethernet 2 header.
    Ethernet2Header,
    /// Error occured in the vlan header.
    VlanHeader,
    /// Error occured when decoding an IP header (v4 or v6).
    IpHeader,
    /// Error occured in the IPv4 layer.
    Ipv4Header,
    /// Error occured verifying the total length of an IPv4 packet.
    Ipv4Packet,
    /// Error occured in the IP authentification header.
    IpAuthHeader,
    /// Error occured in the IPv6 layer.
    Ipv6Header,
    /// Error occured verifying the payload length of an IPv6 packet.
    Ipv6Packet,
    /// Error occured while decoding an IPv6 fragment header.
    Ipv6FragHeader,
    /// Error occured while decoding a generic IPv6 extension header.
    Ipv6ExtHeader,
    /// Error occured while decoding an UDP header.
    UdpHeader,
    /// Error occured verifying the length of the UDP payload.
    UdpPayload,
    /// Error occured while decoding a TCP header.
    TcpHeader,
    /// Error occured while parsing an ICMP packet.
    Icmpv4,
    /// Error occured while parsing an ICMP timestamp packet.
    Icmpv4Timestamp,
    /// Error occured while parsing an ICMP timestamp reply packet.
    Icmpv4TimestampReply,
    /// Error occured while parsing an ICMPv6 packet.
    Icmpv6,
}

impl Layer {
    /// String that is used as a title for the error.
    pub fn error_title(&self) -> &'static str {
        use Layer::*;
        match self {
            Ethernet2Header => "Ethernet 2 Header Error",
            VlanHeader => "VLAN Header Error",
            IpHeader => "IP Header Error",
            Ipv4Header => "IPv4 Header Error",
            Ipv4Packet => "IPv4 Packet Error",
            IpAuthHeader => "IP Authentification Header Error",
            Ipv6Header => "IPv6 Header Error",
            Ipv6Packet => "IPv6 Packet Error",
            Ipv6FragHeader => "IPv6 Fragment Header Error",
            Ipv6ExtHeader => "IPv6 Extension Header Error",
            UdpHeader => "UDP Header Error",
            UdpPayload => "UDP Payload Error",
            TcpHeader => "TCP Header Error",
            Icmpv4 => "ICMP Packet Error",
            Icmpv4Timestamp => "ICMP Timestamp Error",
            Icmpv4TimestampReply => "ICMP Timestamp Reply Error",
            Icmpv6 => "ICMPv6 Packet Error",
        }
    }
}

impl core::fmt::Display for Layer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Layer::*;
        match self {
            Ethernet2Header => write!(f, "Ethernet 2 header"),
            VlanHeader => write!(f, "VLAN header"),
            IpHeader => write!(f, "IP header"),
            Ipv4Header => write!(f, "IPv4 header"),
            Ipv4Packet => write!(f, "IPv4 packet"),
            IpAuthHeader => write!(f, "IP authentification header"),
            Ipv6Header => write!(f, "IPv6 header"),
            Ipv6Packet => write!(f, "IPv6 packet"),
            Ipv6FragHeader => write!(f, "IPv6 fragment header"),
            Ipv6ExtHeader => write!(f, "IPv6 extension header"),
            UdpHeader => write!(f, "UDP header"),
            UdpPayload => write!(f, "UDP payload"),
            TcpHeader => write!(f, "TCP header"),
            Icmpv4 => write!(f, "ICMP packet"),
            Icmpv4Timestamp => write!(f, "ICMP timestamp message"),
            Icmpv4TimestampReply => write!(f, "ICMP timestamp reply message"),
            Icmpv6 => write!(f, "ICMPv6 packet"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Layer::*;
    use alloc::format;
    use std::{
        cmp::{Ord, Ordering},
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn debug() {
        assert_eq!("Ethernet2Header", format!("{:?}", Ethernet2Header));
    }

    #[test]
    fn clone_eq_hash_ord() {
        let layer = Ethernet2Header;
        assert_eq!(layer, layer.clone());
        let hash_a = {
            let mut hasher = DefaultHasher::new();
            layer.hash(&mut hasher);
            hasher.finish()
        };
        let hash_b = {
            let mut hasher = DefaultHasher::new();
            layer.clone().hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash_a, hash_b);
        assert_eq!(Ordering::Equal, layer.cmp(&layer));
        assert_eq!(Some(Ordering::Equal), layer.partial_cmp(&layer));
    }

    #[test]
    fn error_title() {
        let tests = [
            (Ethernet2Header, "Ethernet 2 Header Error"),
            (VlanHeader, "VLAN Header Error"),
            (IpHeader, "IP Header Error"),
            (Ipv4Header, "IPv4 Header Error"),
            (Ipv4Packet, "IPv4 Packet Error"),
            (IpAuthHeader, "IP Authentification Header Error"),
            (Ipv6Header, "IPv6 Header Error"),
            (Ipv6Packet, "IPv6 Packet Error"),
            (Ipv6FragHeader, "IPv6 Fragment Header Error"),
            (Ipv6ExtHeader, "IPv6 Extension Header Error"),
            (UdpHeader, "UDP Header Error"),
            (UdpPayload, "UDP Payload Error"),
            (TcpHeader, "TCP Header Error"),
            (Icmpv4, "ICMP Packet Error"),
            (Icmpv4Timestamp, "ICMP Timestamp Error"),
            (Icmpv4TimestampReply, "ICMP Timestamp Reply Error"),
            (Icmpv6, "ICMPv6 Packet Error"),
        ];
        for test in tests {
            assert_eq!(test.0.error_title(), test.1);
        }
    }

    #[test]
    fn fmt() {
        let tests = [
            (Ethernet2Header, "Ethernet 2 header"),
            (VlanHeader, "VLAN header"),
            (IpHeader, "IP header"),
            (Ipv4Header, "IPv4 header"),
            (Ipv4Packet, "IPv4 packet"),
            (IpAuthHeader, "IP authentification header"),
            (Ipv6Header, "IPv6 header"),
            (Ipv6Packet, "IPv6 packet"),
            (Ipv6FragHeader, "IPv6 fragment header"),
            (Ipv6ExtHeader, "IPv6 extension header"),
            (UdpHeader, "UDP header"),
            (UdpPayload, "UDP payload"),
            (TcpHeader, "TCP header"),
            (Icmpv4, "ICMP packet"),
            (Icmpv4Timestamp, "ICMP timestamp message"),
            (Icmpv4TimestampReply, "ICMP timestamp reply message"),
            (Icmpv6, "ICMPv6 packet"),
        ];
        for test in tests {
            assert_eq!(format!("{}", test.0), test.1);
        }
    }
}
