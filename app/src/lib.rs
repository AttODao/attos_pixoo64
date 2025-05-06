pub mod api;
pub mod models;
pub mod services;

pub mod config;
pub mod consts;
pub mod errors;

#[cfg(test)]
mod tests {
  use crate::{api::news_api::get_news, models::news_response::NewsResponse};

  #[test]
  fn pixoo_api_test() {}

  #[tokio::test]
  async fn news_api_test() {
    let res = get_news().await;
    println!("{:?}", res);
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn parse_json_test() {
    let json = r#"{
"status": "ok",
"totalResults": 36,
"articles": [
{
"source": {
"id": null,
"name": "SciTechDaily"
},
"author": null,
"title": "The Magnetar That Forged a Planet’s Worth of Gold in Half a Second - SciTechDaily",
"description": "A powerful cosmic event from 2004—a gamma-ray burst from a magnetar 30,000 light-years away—has just been revealed as a major source of the universe’s heaviest elements like gold and platinum. Researchers have shown that this intense flare, brighter than anyt…",
"url": "https://scitechdaily.com/the-magnetar-that-forged-a-planets-worth-of-gold-in-half-a-second/",
"urlToImage": "https://scitechdaily.com/images/Star-Explosion-Heavy-Elements-Art.jpg",
"publishedAt": "2025-05-01T08:25:57Z",
"content": "One of the brightest explosions in our galaxy may have forged more heavy elements than we ever imagined. The 2004 magnetar flare is now confirmed to be a cosmic forge of gold, platinum, and other rar… [+6199 chars]"
},
{
"source": {
"id": "axios",
"name": "Axios"
},
"author": "Rebecca Falconer",
"title": "Split screen: Harris slams Trump \"chaos,\" president says he's made no mistakes - Axios",
"description": "Harris praised those who've defied Trump's \"unconstitutional demands\" during a major speech in California.",
"url": "https://www.axios.com/2025/05/01/kamala-harris-speech-trump-100-days",
"urlToImage": "https://images.axios.com/q68pNkEvsJCHEAsF_7xDLRYXsGU=/0x0:1920x1080/1366x768/2025/05/01/1746075442392.jpg",
"publishedAt": "2025-05-01T06:56:15Z",
"content": "Former Vice President Harris accused President Trump of being responsible for the \"greatest man-made economic crisis in modern presidential history\" during a Wednesday night speech in San Francisco, … [+2990 chars]"
},
{
"source": {
"id": null,
"name": "CNBC"
},
"author": "Amala Balakrishner",
"title": "Tesla shares fall on report that company’s board seeks new CEO to replace Elon Musk - CNBC",
"description": "Tesla chair Robyn Denholm wrote on the social media platform X that the WSJ report was \"absolutely false.\"",
"url": "https://www.cnbc.com/2025/05/01/tesla-shares-fall-on-report-that-companys-board-seeks-new-ceo-to-replace-elon-musk.html",
"urlToImage": "https://image.cnbcfm.com/api/v1/image/108134942-17453443092025-04-10t154750z_252589264_rc2mida9foxe_rtrmadp_0_usa-trump-musk-congress.jpeg?v=1745344380&w=1920&h=1080",
"publishedAt": "2025-05-01T06:34:48Z",
"content": "Shares of Tesla were flat in premarket trading Thursday after the EV maker denied a Wall Street Journal report that its board was searching for a replacement for chief executive Elon Musk.\r\nThe repor… [+2044 chars]"
},
{
"source": {
"id": "cnn",
"name": "CNN"
},
"author": "Victoria Butenko, Kit Maher, Ivana Kottasová, Daria Tarasova-Markina",
"title": "US and Ukraine sign critical minerals deal - CNN",
"description": "The United States and Ukraine have signed an “economic partnership agreement” that will give Washington access to Kyiv’s rare earth minerals in exchange for establishing an investment fund in Ukraine.",
"url": "https://www.cnn.com/2025/04/30/europe/ukraine-us-mineral-deal-intl/index.html",
"urlToImage": "https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2201494264.jpg?c=16x9&q=w_800,c_fill",
"publishedAt": "2025-05-01T06:26:00Z",
"content": "The United States and Ukraine have signed an economic partnership agreement that will give Washington access to Kyivs rare earth minerals in exchange for establishing an investment fund in Ukraine.\r\n… [+5350 chars]"
},
{
"source": {
"id": null,
"name": "Suntimes.com"
},
"author": "Georgia Nicols",
"title": "Horoscope for Thursday, May 01, 2025 - Chicago Sun-Times",
"description": null,
"url": "https://chicago.suntimes.com/horoscopes/2025/05/01/horoscopes-today-thursday-may-1-2025",
"urlToImage": "https://cst.brightspotcdn.com/dims4/default/2145dbd/2147483647/strip/true/crop/870x497+0+67/resize/1461x834!/quality/90/?url=https%3A%2F%2Fchorus-production-cst-web.s3.us-east-1.amazonaws.com%2Fbrightspot%2Fac%2Ffd%2F790f04b15195427014757adc0272%2Fgeorgia-nicols.jpg",
"publishedAt": "2025-05-01T05:03:28Z",
"content": "Moon alert\r\nAfter 2:45 a.m. Chicago time, there are no restrictions to shopping or important decisions. The moon is in Cancer.\r\nAries (March 21-April 19)\r\nBe aware that conversations with family memb… [+3908 chars]"
},
{
"source": {
"id": null,
"name": "New York Post"
},
"author": "Christian Arnold",
"title": "Bill Belichick’s daughter-in-law shades Jordon Hudson as CBS interview drama grows - New York Post",
"description": "Bill Belichick’s daughter-in-law weighed in on the ongoing drama involving his 24-year-old girlfriend, Jordan Hudson the famed football coach and his “CBS Sunday Morning” interview.",
"url": "https://nypost.com/2025/05/01/sports/bill-belichicks-daughter-in-law-shades-jordon-hudson-over-cbs-interview/",
"urlToImage": "https://nypost.com/wp-content/uploads/sites/2/2025/04/newspress-collage-z4evroe05-1746070448663.jpg?quality=75&strip=all&1746056211&w=1024",
"publishedAt": "2025-05-01T04:02:00Z",
"content": "Bill Belichicks daughter-in-law weighed in on the ongoing drama involving his 24-year-old girlfriend Jordan Hudson, the famed football coach and his CBS Sunday Morning interview. \r\nJennifer Schmitt, … [+3336 chars]"
},
{
"source": {
"id": null,
"name": "BBC News"
},
"author": null,
"title": "Mushroom trial: 'very rare' for accused to hold gatherings, says ex-husband - BBC",
"description": "Three people died in hospital after being poisoned with death cap mushrooms.",
"url": "https://www.bbc.com/news/articles/c2dek63gxp6o",
"urlToImage": "https://ichef.bbci.co.uk/news/1024/branded_news/1f06/live/21dbe990-2643-11f0-a20d-1d8f85937283.jpg",
"publishedAt": "2025-05-01T03:18:47Z",
"content": "Simon Atkinson and Katy Watson\r\nSimon Patterson was invited to the fatal meal prepared by Erin Patterson - but decided not to attend the day before\r\nThe estranged husband of a woman who served a pois… [+7289 chars]"
},
{
"source": {
"id": null,
"name": "Hollywood Reporter"
},
"author": "Pamela McClintock",
"title": "Box Office: Chris Hemsworth, Halle Berry’s ‘Crime 101’ Lands Presidents’ Day Weekend 2026 Release (Exclusive) - The Hollywood Reporter",
"description": "The L.A.-set heist movie also stars Mark Ruffalo, Barry Keoghan, Monica Barbaro, Corey Hawkins, Jennifer Jason Leigh and Nick Nolte.",
"url": "http://www.hollywoodreporter.com/movies/movie-news/chris-hemsworth-halle-berry-crime-101-2026-release-1236204739/",
"urlToImage": "https://www.hollywoodreporter.com/wp-content/uploads/2025/04/Halle-Berry-Chris-Hemsworth-getty-H-2025.png?w=1296&h=730&crop=1",
"publishedAt": "2025-05-01T02:20:24Z",
"content": "Chris Hemsworth and Halle Berry‘s heist movie Crime 101 has landed a high-profile release date in theaters.\r\nAmazon MGM Studios will open the film on Feb. 13, the beginning of the long Valentine’s Da… [+673 chars]"
},
{
"source": {
"id": null,
"name": "NPR"
},
"author": "",
"title": "May Day protesters will rally nationwide against the 'war on working people' - NPR",
"description": "May Day is not officially acknowledged in the U.S. because of what historians say is an ongoing resistance to unity among the working class. This resistance is prompting protesters to take action on Thursday, regardless.",
"url": "https://www.npr.org/2025/04/30/nx-s1-5382560/may-day-protests-history-trump",
"urlToImage": "https://npr.brightspotcdn.com/dims3/default/strip/false/crop/2485x1398+0+308/resize/1400/quality/100/format/jpeg/?url=http%3A%2F%2Fnpr-brightspot.s3.amazonaws.com%2F0c%2F4e%2F328520d34570beea14baf107aab1%2Fap18121187248316.jpg",
"publishedAt": "2025-05-01T01:45:08Z",
"content": "Tens of thousands of protesters are expected to take to the streets nationwide on Thursday in May Day rallies opposing the Trump administration.\r\nMay Day, celebrated by workers across the globe as In… [+4130 chars]"
},
{
"source": {
"id": "politico",
"name": "Politico"
},
"author": "Liz Crampton, Kyle Cheney",
"title": "Judge frees Columbia student activist whom Trump administration wants to deport - Politico",
"description": "Mohsen Mahdawi was arrested as part of a crackdown on pro-Palestinian students who were legally studying in the United States.",
"url": "https://www.politico.com/news/2025/04/30/judge-frees-columbia-student-activist-whom-trump-administration-wants-to-deport-00317981",
"urlToImage": "https://static.politico.com/3b/a9/2eb70610459cb6d5830b490eb124/img-4822.jpg",
"publishedAt": "2025-05-01T01:25:28Z",
"content": "I am saying it clear and loud, Mahdawi said outside a federal courthouse shortly after U.S. District Judge Geoffrey Crawford , an Obama appointee, ordered his release. To President Trump and his Cabi… [+3853 chars]"
},
{
"source": {
"id": "the-washington-post",
"name": "The Washington Post"
},
"author": "Rachel Lerman",
"title": "Here’s how GDP shows tariffs are starting to take a toll - The Washington Post",
"description": "Few pieces of the first-quarter GDP report remained untouched by the Trump administration’s tariff policy.",
"url": "https://www.washingtonpost.com/business/2025/04/30/gdp-tariff-economy-consumer-spending/",
"urlToImage": "https://www.washingtonpost.com/wp-apps/imrs.php?src=https://arc-anglerfish-washpost-prod-washpost.s3.amazonaws.com/public/5OQQIYEZGFB3RDCVAS4SOBL4ZI.jpg&w=1440",
"publishedAt": "2025-05-01T00:31:48Z",
"content": "The much-anticipated report on the nations economy released Wednesday showed that the U.S. economy shrank during the first three months of the year offering the first window into the widespread effec… [+696 chars]"
},
{
"source": {
"id": "cnn",
"name": "CNN"
},
"author": "Kit Maher",
"title": "Trump Cabinet gives Musk a seemingly friendly send-off — at odds with his tumultuous tenure - CNN",
"description": "Cabinet secretaries were largely complimentary of the billionaire, even applauding him at one point, despite past tensions.",
"url": "https://www.cnn.com/2025/04/30/politics/musk-cabinet-meeting-farewell",
"urlToImage": "https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2212146103.jpg?c=16x9&q=w_800,c_fill",
"publishedAt": "2025-05-01T00:12:00Z",
"content": "At what might have been his last Cabinet meeting for the foreseeable future, billionaire presidential adviser Elon Musk donned two baseball caps, one stacked atop the other. The first, emblazoned wit… [+4353 chars]"
},
{
"source": {
"id": null,
"name": "NBCSports.com"
},
"author": "Mike Florio",
"title": "NFL sent Shedeur Sanders’s phone number directly to Jeff Ulbrich - NBC Sports",
"description": "The league, not the Falcons, gave Ulbrich the number.",
"url": "https://www.nbcsports.com/nfl/profootballtalk/rumor-mill/news/nfl-sent-shedeur-sanderss-phone-number-directly-to-jeff-ulbrich",
"urlToImage": "https://nbcsports.brightspotcdn.com/dims4/default/cfd079f/2147483647/strip/true/crop/4256x2394+0+0/resize/1440x810!/quality/90/?url=https%3A%2F%2Fnbc-sports-production-nbc-sports.s3.us-east-1.amazonaws.com%2Fbrightspot%2Ff3%2Ff2%2Fa5b37861405f9827338d94bcaed2%2Fhttps-api-imagn.com%2Frest%2Fdownload%2FimageID%3D25048366",
"publishedAt": "2025-04-30T23:38:57Z",
"content": "In the aftermath of the NFLs decision to fine the Falcons $250,000 and defensive coordinator Jeff Ulbrich $100,000 for failing to prevent the disclosure of confidential information distributed to the… [+2035 chars]"
},
{
"source": {
"id": "nbc-news",
"name": "NBC News"
},
"author": "Scott Wong, Melanie Zanona",
"title": "Republicans hit early snags as they start crafting a massive bill for Trump's agenda - NBC News",
"description": "Republicans are already hitting some snags as they begin the work of crafting a bill for President Donald Trump’s sweeping domestic policy agenda.",
"url": "https://www.nbcnews.com/politics/congress/republicans-hit-early-snags-start-crafting-massive-bill-trumps-agenda-rcna203746",
"urlToImage": "https://media-cldnry.s-nbcnews.com/image/upload/t_nbcnews-fp-1200-630,f_auto,q_auto:best/rockcms/2025-04/250430-mike-johnson-mn-1120-a579c6.jpg",
"publishedAt": "2025-04-30T23:17:00Z",
"content": "WASHINGTON Republicans are already hitting some snags as they begin the work of crafting a bill for President Donald Trumps sweeping domestic policy agenda. And they havent even made some of their ha… [+7779 chars]"
},
{
"source": {
"id": null,
"name": "hoopsrumors.com"
},
"author": null,
"title": "Bucks Notes: Giannis, Rivers, LaVine, Horst - Hoops Rumors",
"description": "Giannis Antetokounmpo will meet with Bucks management soon as he faces a decision that could change the course of the franchise.",
"url": "https://www.hoopsrumors.com/2025/04/bucks-notes-giannis-rivers-lavine-horst.html",
"urlToImage": "https://cdn.hoopsrumors.com/files/2024/11/Giannis-Antetokounmpo-1-900x600.jpg",
"publishedAt": "2025-04-30T23:14:00Z",
"content": "Now that the Bucks‘ season is over, Giannis Antetokounmpo will meet with management to discuss plans for his future and the team’s, writes Jamal Collier of ESPN, citing sources who spoke with the net… [+3821 chars]"
},
{
"source": {
"id": null,
"name": "BBC News"
},
"author": null,
"title": "Polls to open for local elections in England - BBC",
"description": "Hundreds of councillors, six mayors and one MP will be elected in polls across England.",
"url": "https://www.bbc.com/news/articles/crm3rl27k8lo",
"urlToImage": "https://ichef.bbci.co.uk/news/1024/branded_news/ca6e/live/1587de90-25f0-11f0-a2c2-9525794a9c0a.jpg",
"publishedAt": "2025-04-30T23:00:05Z",
"content": "Voters are set to head to the polls for local and mayoral elections being held in some parts of England on Thursday.\r\nThere are local elections to 24 of England's 317 councils and six mayoral authori… [+2240 chars]"
},
{
"source": {
"id": null,
"name": "Deadline"
},
"author": "Armando Tinoco",
"title": "'The View's Ana Navarro Praises ABC News' Terry Moran After Donald Trump Interview, Saying He \"Gave A Masterclass In Journalism\" - Deadline",
"description": "Ana Navarro is praising her ABC News colleague Terry Moran following his contentious interview with Donald Trump.",
"url": "http://deadline.com/2025/04/the-view-ana-navarro-praises-abc-news-terry-moran-donald-trump-interview-1236381801/",
"urlToImage": "https://deadline.com/wp-content/uploads/2025/04/ana-navarro-terry-moran-donald-trump.jpg?w=1024",
"publishedAt": "2025-04-30T22:15:00Z",
"content": "Ana Navarro is praising her ABC News colleague Terry Moran following his contentious interview with Donald Trump.\r\nThe political analyst took to social media to highlight Moran’s skills and not backi… [+1516 chars]"
},
{
"source": {
"id": "nbc-news",
"name": "NBC News"
},
"author": "Berkeley Lovelace Jr.",
"title": "Wegovy treated a serious form of liver disease in a major clinical trial - NBC News",
"description": "The weight loss drug Wegovy treated a serious form of liver disease in about two-thirds of patients in a major clinical trial, according to the findings published Wednesday in the New England Journal of Medicine.",
"url": "https://www.nbcnews.com/health/health-news/wegovy-treated-serious-form-liver-disease-major-clinical-trial-rcna203677",
"urlToImage": "https://media-cldnry.s-nbcnews.com/image/upload/t_nbcnews-fp-1200-630,f_auto,q_auto:best/rockcms/2024-05/240514-wegovy-ch-1040-a1f5f2.jpg",
"publishedAt": "2025-04-30T21:17:18Z",
"content": "The weight loss drug Wegovy treated a serious form of liver disease in about two-thirds of patients in a major clinical trial, according to the findings published Wednesday in the New England Journal… [+5459 chars]"
},
{
"source": {
"id": null,
"name": "Android Headlines"
},
"author": "Justin Diaz",
"title": "Leaked image teases Samsung's \"beyond slim\" phone for mid-May launch - Android Headlines",
"description": "The launch of the Galaxy S25 Edge may have just been confirmed by Samsung unofficially, thanks to the leak of a new teaser for an upcoming event. This",
"url": "https://www.androidheadlines.com/2025/04/leaked-image-teases-samsungs-beyond-slim-phone-for-mid-may-launch.html",
"urlToImage": "https://www.androidheadlines.com/wp-content/uploads/2025/03/AH-Samsung-Galaxy-S25-Edge-1.jpg",
"publishedAt": "2025-04-30T21:14:34Z",
"content": "The launch of the Galaxy S25 Edge may have just been confirmed by Samsung unofficially, thanks to the leak of a new teaser for an upcoming event. This also isnt the first time the launch of Samsungs … [+2018 chars]"
}
]
}"#;
    let json: NewsResponse = serde_json::from_str(json).unwrap();
    match json {
      NewsResponse::Ok {
        total_results,
        articles,
      } => {
        println!("total_results: {}", total_results);
        for article in articles {
          println!("title: {}", article.title);
        }
      }
      NewsResponse::Error { code, message } => {
        println!("code: {}, message: {}", code, message);
      }
    }
    assert!(true);
  }
}
