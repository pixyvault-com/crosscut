/// host is the natural unique identifier for a hosting device
struct HostId(String);

/// consumer is the natural unique identifier for a consumer (an email or phone number)
enum ConsumerId {
    Sms(phonenumber::PhoneNumber),
    Email(email_address::EmailAddress),
}

trait TotpChallengeSender {
    type Error;

    fn challenge(&self, otp: i32) -> Result<Self::Error, ()>;
}

impl TotpChallengeSender for phonenumber::PhoneNumber {
    type Error = ();

    fn challenge(&self, otp: i32) -> Result<Self::Error, ()> {
        // sns?
        Ok(())
    }
}

impl TotpChallengeSender for email_address::EmailAddress {
    type Error = ();

    fn challenge(&self, otp: i32) -> Result<Self::Error, ()> {
        // ses?
        Ok(())
    }
}

///

trait PushNotifier {
    type Identifier;
    type Error;

    fn notify(id: &Self::Identifier, inverter_url: &url::Url) -> Result<Self::Error, ()>;
}

struct ApplePushNotificationServiceDeviceToken;

struct ApplePushNotificationService;

impl PushNotifier for ApplePushNotificationService {
    type Identifier = ApplePushNotificationServiceDeviceToken;
    type Error = ();

    fn notify(id: &Self::Identifier, inverter_url: &url::Url) -> Result<Self::Error, ()> {
        // apns
        Ok(())
    }
}

///

fn register_host(host: &HostId, identifier: ApplePushNotificationServiceDeviceToken) -> () {
    // validation: server is unique, dns-friendly
    // persist to postgres
}

fn handle_pseudo_push_notification_backchannel(host: &HostId, stream: std::net::TcpStream) -> () {
    loop {
    }
}

fn register_totp_secret(host: &HostId, client: &ConsumerId, secret: &otp::Secret) -> () {
    // persist to postgres **encrypted**
}

fn handle_proxy(host: &HostId, client: &ConsumerId, consumer_stream: std::net::TcpStream) -> () {
    // fetch totp secret for (HostId, ConsumerId) (or 404)
    // valid cookie authenticated:
    //   send push notification on best channel to host
    //   park
    // valid basic (totp) authenticated:
    //   add a `Set-Cookie` to the response
    //   send push notification on best channel to host
    //   park
    // else:
    //   send one-time code on appropriate channel
    //   return 401
}

fn handle_inverter(host: &HostId, client: &ConsumerId, host_stream: std::net::TcpStream) -> () {
    // authenticate the host (???)
    // connect host_stream to consumer_stream (linux splice?)
}

////////

fn main() {
    println!("Hello, world!");
}
