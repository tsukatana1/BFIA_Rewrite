const img = document.getElementById("fam-pic");

const tl = new TimelineMax();
const aos = AOS.init({
    offset: 300,
    duration: 600
});

tl.fromTo(img, 1.25, { height: "0%"}, { height: "100%", ease: Power2.easeInOut })