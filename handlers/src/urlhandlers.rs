use std::fmt;
use url::Url as OtherUrl;

struct Url {
    url: OtherUrl,

}

trait UrlHandler {
    fn build(url: &str) -> Url {
        Url {
            url: OtherUrl::parse(url).unwrap(),
        }
    }

    fn get_website_title(&self) -> String {
        todo!();
    }

    fn as_url(&self) -> String;
}

impl UrlHandler for Url  {

    fn as_url(&self) -> String {
        self.url.to_string()
    }

}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generic url: {}", self.url)
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    pub fn test_generic_url() {
        let myurl = "https://onlypans.com/pepe";
        let the_response = Url::build(myurl);
        let expected = "Generic url: https://onlypans.com/pepe";
        assert_eq!(expected, the_response.to_string());
    }

    #[test]
    pub fn test_generic_as_url() {
        let myurl = "https://onlYpans.com/pepe";
        let the_response = Url::build(myurl);
        let expected = "https://onlypans.com/pepe";
        assert_eq!(expected, the_response.as_url());
    }

    #[test]
    pub fn test_onlyfans_url() {
        let myurl = "https://OnlyFans.com/pepe";
        let the_response = Url::build(myurl);
        let expected = "Generic url: https://onlyfans.com/pepe";
        assert_eq!(expected, the_response.to_string());
    }

        #[test]
    pub fn test_facebook_url() {
        let myurl = "https://facebook.com/pepe";
        let the_response = Url::build(myurl);
        let expected = "Generic url: https://facebook.com/pepe";
        assert_eq!(expected, the_response.to_string());
    }

}