pub mod cache;
pub mod commands;
pub mod configuration;
pub mod errors;
pub mod http;
pub mod shared;

use log;

#[cfg(test)]
mod tests {
    use crate::{commands, shared};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn parse_star_count() {
        let test_response = TEST_RESPONSE.to_string();
        let result = commands::parse_star_count_from_response(test_response, 19).unwrap();
        assert_eq!(result, 2);
    }

    const TEST_RESPONSE: &str = r#"
<!DOCTYPE html>
<html lang="en#-us">
<head>
<meta charset="utf-8"/>
<title>Advent of Code 2020</title>
<!--[if lt IE 9]><script src="/static/html5.js"></script><![endif]-->
<link href='//fonts.googleapis.com/css?family=Source+Code+Pro:300&subset=latin,latin-ext' rel='stylesheet' type='text/css'/>
<link rel="stylesheet" type="text/css" href="/static/style.css?28"/>
<link rel="stylesheet alternate" type="text/css" href="/static/highcontrast.css?0" title="High Contrast"/>
<link rel="shortcut icon" href="/favicon.png"/>
<script>window.addEventListener('click', function(e,s,r){if(e.target.nodeName==='CODE'&&e.detail===3){s=window.getSelection();s.removeAllRanges();r=document.createRange();r.selectNodeContents(e.target);s.addRange(r);}});</script>
</head><!--




Oh, hello!  Funny seeing you here.

I appreciate your enthusiasm, but you aren't going to find much down here.
There certainly aren't clues to any of the puzzles.  The best surprises don't
even appear in the source until you unlock them for real.

Please be careful with automated requests; I'm not a massive company, and I can
only take so much traffic.  Please be considerate so that everyone gets to play.

If you're curious about how Advent of Code works, it's running on some custom
Perl code. Other than a few integrations (auth, analytics, social media), I
built the whole thing myself, including the design, animations, prose, and all
of the puzzles.

The puzzles are most of the work; preparing a new calendar and a new set of
puzzles each year takes all of my free time for 4-5 months. A lot of effort
went into building this thing - I hope you're enjoying playing it as much as I
enjoyed making it for you!

If you'd like to hang out, I'm @ericwastl on Twitter.

- Eric Wastl


















































-->
<body>
<header><div><h1 class="title-global"><a href="/">Advent of Code</a></h1><nav><ul><li><a href="/2020/about">[About]</a></li><li><a href="/2020/events">[Events]</a></li><li><a href="https://teespring.com/stores/advent-of-code" target="_blank">[Shop]</a></li><li><a href="/2020/settings">[Settings]</a></li><li><a href="/2020/auth/logout">[Log Out]</a></li></ul></nav><div class="user">lasse16 <span class="star-count">48*</span></div></div><div><h1 class="title-event">&nbsp;&nbsp;<span class="title-event-wrap">0.0.0.0:</span><a href="/2020">2020</a><span class="title-event-wrap"></span></h1><nav><ul><li><a href="/2020">[Calendar]</a></li><li><a href="/2020/support">[AoC++]</a></li><li><a href="/2020/sponsors">[Sponsors]</a></li><li><a href="/2020/leaderboard">[Leaderboard]</a></li><li><a href="/2020/stats">[Stats]</a></li></ul></nav></div></header>

<div id="sidebar">
<div id="sponsor"><div class="quiet">Our <a href="/2020/sponsors">sponsors</a> help make Advent of Code possible:</div><div class="sponsor"><a href="https://smartystreets.com/aoc" target="_blank" onclick="if(ga)ga('send','event','sponsor','sidebar',this.href);" rel="noopener">SmartyStreets</a> - - -- Ridiculously - ----- Fast ------ ---- Address ---- -- Verification - ------ AND ------ ---- Rooftop ---- --- Geocoding ---</div></div>
</div><!--/sidebar-->

<main>
<style>
.calendar .calendar-color-b { color:#333399; }
.calendar .calendar-color-w { color:#ffffff; }
.calendar .calendar-color-g { color:#00cc00; }
.calendar .calendar-color-r { color:#ff0000; }
.calendar .calendar-color-l { color:#ccccff; }
.calendar .calendar-color-a { color:#cccccc; }
.calendar .calendar-color-o { color:#ffcc99; }
</style>
<pre class="calendar"><a aria-label="Day 1, two stars" href="/2020/day/1" class="calendar-day1 calendar-verycomplete">              <span class="calendar-color-l">..........</span><span class="calendar-color-r">|</span><span class="calendar-color-l">..........</span>                <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 2, two stars" href="/2020/day/2" class="calendar-day2 calendar-verycomplete">   <span class="calendar-color-l">.....''''''</span> <span class="calendar-color-w">.'</span>  <span class="calendar-color-w">-</span>  <span class="calendar-color-w">-</span>  <span class="calendar-color-a">\</span><span class="calendar-color-w">-</span> <span class="calendar-color-w">.''</span><span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-l">''''''.....</span>     <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 3, two stars" href="/2020/day/3" class="calendar-day3 calendar-verycomplete"><span class="calendar-color-l">'''</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-w">'.'.</span> <span class="calendar-color-w">-</span>   <span class="calendar-color-w">-</span> <span class="calendar-color-a">\</span> <span class="calendar-color-w">-'':</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-l">'''</span>  <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 4, two stars" href="/2020/day/4" class="calendar-day4 calendar-verycomplete"> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-w">''..'''</span><span class="calendar-color-a">_[]</span><span class="calendar-color-w">.'</span>  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>   <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 5, two stars" href="/2020/day/5" class="calendar-day5 calendar-verycomplete"><span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-g">.'.</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-a">____/</span> <span class="calendar-color-w">''</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  ~  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>  <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 6, two stars" href="/2020/day/6" class="calendar-day6 calendar-verycomplete">  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span> <span class="calendar-color-g">''</span>  <span class="calendar-color-g">..</span><span class="calendar-color-a">_____/</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>    _ <span class="calendar-color-b">~</span> _   O&gt;    <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 7, two stars" href="/2020/day/7" class="calendar-day7 calendar-verycomplete"> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-g">:</span><span class="calendar-color-a">[]</span><span class="calendar-color-g">'.</span>   <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>  \ / \ / \ /  <span class="calendar-color-b">~</span>   <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 8, two stars" href="/2020/day/8" class="calendar-day8 calendar-verycomplete">       <span class="calendar-color-b">~</span>     <span class="calendar-color-g">'.</span><span class="calendar-color-a">\</span> <span class="calendar-color-b">~</span>        <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>   ~  <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>    ~  <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 17, two stars" href="/2020/day/17" class="calendar-day17 calendar-verycomplete"> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>   <span class="calendar-color-a">\</span>  <span class="calendar-color-b">~</span>   <span class="calendar-color-a">____</span>     <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>    <span class="calendar-day">17</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 9, two stars" href="/2020/day/9" class="calendar-day9 calendar-verycomplete">       <span class="calendar-color-b">~</span>       <span class="calendar-color-b">~</span> <span class="calendar-color-a">\____/</span> <span class="calendar-color-g">.''</span><span class="calendar-color-a">\</span><span class="calendar-color-g">'..</span>    <span class="calendar-color-b">~</span>       <span class="calendar-color-b">~</span>    <span class="calendar-color-g">.</span>  <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 16, two stars" href="/2020/day/16" class="calendar-day16 calendar-verycomplete">    <span class="calendar-color-b">~</span>       <span class="calendar-color-b">~</span>  <span class="calendar-color-a">__/\</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-g">.'</span><span class="calendar-color-w">^</span> <span class="calendar-color-a">[]</span><span class="calendar-color-g">.'</span>      <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-g">..''</span>   <span class="calendar-day">16</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 18, two stars" href="/2020/day/18" class="calendar-day18 calendar-verycomplete"><span class="calendar-color-g">...</span>     <span class="calendar-color-b">~</span><span class="calendar-color-a">_____/</span><span class="calendar-color-b">~</span>   <span class="calendar-color-a">\</span>    <span class="calendar-color-g">:</span><span class="calendar-color-w">^</span> <span class="calendar-color-g">,</span> <span class="calendar-color-g">:</span><span class="calendar-color-a">\</span>  <span class="calendar-color-b">~</span>       <span class="calendar-color-g">:''</span>  <span class="calendar-color-g">,</span> <span class="calendar-color-w">^</span>  <span class="calendar-day">18</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 15, two stars" href="/2020/day/15" class="calendar-day15 calendar-verycomplete"><span class="calendar-color-g">###:</span> <span class="calendar-color-g">...</span><span class="calendar-color-a">/</span>   <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span><span class="calendar-color-a">\</span>    <span class="calendar-color-g">'..'</span>  <span class="calendar-color-a">\_______</span><span class="calendar-color-b">~</span> <span class="calendar-color-g">'.</span> <span class="calendar-color-g">,</span>      <span class="calendar-day">15</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 19, two stars" href="/2020/day/19" class="calendar-day19 calendar-verycomplete"><span class="calendar-color-g">.''</span> <span class="calendar-color-g">.'</span><span class="calendar-color-a">[]</span><span class="calendar-color-g">'.</span>           <span class="calendar-color-a">\</span> <span class="calendar-color-b">~</span>       <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>   <span class="calendar-color-a">\</span>  <span class="calendar-color-g">:</span>  <span class="calendar-color-g">,</span> <span class="calendar-color-w">^</span>   <span class="calendar-day">19</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 10, two stars" href="/2020/day/10" class="calendar-day10 calendar-verycomplete"><span class="calendar-color-g">'...'##</span><span class="calendar-color-a">\</span><span class="calendar-color-g">##'.</span>  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>  <span class="calendar-color-a">\</span>   <span class="calendar-color-b">~</span>            <span class="calendar-color-g">.</span><span class="calendar-color-a">\</span><span class="calendar-color-g">'</span> <span class="calendar-color-g">,</span>  <span class="calendar-color-w">^</span> <span class="calendar-color-w">^</span>  <span class="calendar-day">10</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 14, two stars" href="/2020/day/14" class="calendar-day14 calendar-verycomplete"><span class="calendar-color-g">#####</span> <span class="calendar-color-g">,#</span><span class="calendar-color-a">\</span><span class="calendar-color-g">#.'</span>           <span class="calendar-color-a">\</span>   <span class="calendar-color-g">.</span>       <span class="calendar-color-b">~</span>   <span class="calendar-color-g">'.</span><span class="calendar-color-a">[]</span>  <span class="calendar-color-g">,</span> <span class="calendar-color-w">^</span>   <span class="calendar-day">14</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 20, one star" href="/2020/day/20" class="calendar-day20 calendar-complete"><span class="calendar-color-g">#,</span>      ,<span class="calendar-color-a">\</span><span class="calendar-color-g">'.</span>         <span class="calendar-color-b">~</span>  <span class="calendar-color-a">\</span><span class="calendar-color-g">'',:</span>  <span class="calendar-color-b">~</span>       <span class="calendar-color-a">_/</span><span class="calendar-color-g">'..</span>  <span class="calendar-color-g">,</span> <span class="calendar-color-w">^</span>  <span class="calendar-day">20</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 11, two stars" href="/2020/day/11" class="calendar-day11 calendar-verycomplete">   ~ ~   ,<span class="calendar-color-a">\</span>,<span class="calendar-color-g">'</span>.   ~     <span class="calendar-color-g">:</span><span class="calendar-color-a">[]</span><span class="calendar-color-g">..'</span>         <span class="calendar-color-a">/</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-g">''...</span>  <span class="calendar-day">11</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 13, two stars" href="/2020/day/13" class="calendar-day13 calendar-verycomplete"><span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> ~ ~    <span class="calendar-color-a">\</span>, :         <span class="calendar-color-g">''</span><span class="calendar-color-a">\__</span>  <span class="calendar-color-b">~</span>     <span class="calendar-color-a">/</span>             <span class="calendar-day">13</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 21, two stars" href="/2020/day/21" class="calendar-day21 calendar-verycomplete"> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> ~ ~ <span class="calendar-color-g">,</span><span class="calendar-color-a">[]</span><span class="calendar-color-g">:</span>     <span class="calendar-color-b">~</span>     <span class="calendar-color-b">~</span>  <span class="calendar-color-a">\__</span>    <span class="calendar-color-a">/</span><span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>      <span class="calendar-day">21</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 12, two stars" href="/2020/day/12" class="calendar-day12 calendar-verycomplete"><span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span>  <span class="calendar-color-g">,</span> <span class="calendar-color-g">.</span><span class="calendar-color-a">\______</span>           <span class="calendar-color-a">\__/</span>               <span class="calendar-day">12</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 22, two stars" href="/2020/day/22" class="calendar-day22 calendar-verycomplete"> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span> <span class="calendar-color-o">~</span>    <span class="calendar-color-g">..'</span>   <span class="calendar-color-b">~</span>    <span class="calendar-color-a">\______</span>    <span class="calendar-color-b">~</span>         <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>   <span class="calendar-day">22</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 23, two stars" href="/2020/day/23" class="calendar-day23 calendar-verycomplete">  <span class="calendar-color-g">.....'''</span>           <span class="calendar-color-b">~</span>     <span class="calendar-color-b">~</span><span class="calendar-color-a">\____</span>                  <span class="calendar-day">23</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 24, two stars" href="/2020/day/24" class="calendar-day24 calendar-verycomplete"><span class="calendar-color-g">''</span>         <span class="calendar-color-b">~</span>                 <span class="calendar-color-g">.'..</span><span class="calendar-color-a">\___</span><span class="calendar-color-g">''..</span>       <span class="calendar-color-b">~</span>  <span class="calendar-day">24</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
<a aria-label="Day 25, one star" href="/2020/day/25" class="calendar-day25 calendar-complete">     <span class="calendar-color-b">~</span>          <span class="calendar-color-b">~</span>        <span class="calendar-color-b">~</span>    <span class="calendar-color-g">'.'</span>  <span class="calendar-color-g">:</span> <span class="calendar-color-a">[]</span> .'  <span class="calendar-color-b">~</span>      <span class="calendar-day">25</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
</pre>
</main>

<!-- ga -->
<script>
(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
})(window,document,'script','//www.google-analytics.com/analytics.js','ga');
ga('create', 'UA-69522494-1', 'auto');
ga('set', 'anonymizeIp', true);
ga('send', 'pageview');
</script>
<!-- /ga -->
</body>
</html>
"#;
}
