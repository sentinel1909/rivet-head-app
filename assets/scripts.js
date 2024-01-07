// assets/scripts.js

'use strict';

// data for the diary entries
const card_data = [
  {
    Date: "Monday, January 1st",
    Band: "Judas Priest",
    Album: "Invincible Shield",
    Thoughts: "Two songs out so far, literally cannot wait for the full album in April"
  },
  {
    Date: "Tuesday, January 2nd",
    Band: "Metallica",
    Album: "72 Seasons",
    Thoughts: "Their best, most consistent work in years."
  },
  {
    Date: "Wednesday, January 3rd",
    Band: "Sabaton",
    Album: "Attero Dominatus",
    Thoughts: "Introduced to Sabaton by a co-worker in early 2021, this is my favourite album by far."
  },
  {
    Date: "Thursday, January 4th",
    Band: "Alice Cooper",
    Album: "Special Forces",
    Thoughts: "Discovered this album early in 2021, after exploring new old things. So good, from front to back."
  },
  {
    Date: "Thursday, January 4th",
    Band: "Danko Jones",
    Album: "Born a Lion",
    Thoughts: "Had a brief obsession with these guys, a few years back. Their first album I found to be their best. Straight up crunchy riffs and great beat."
  }
];

/**
 * Create an HTML string from the card data
 * @return (String) The HTML string
 */

function diary_entrie() {
  return card_data.map(card => `<article><h3>${card.Date}</h3><h4>${card.Band}</h4><p>${card.Album}</p><p>${card.Thoughts}</p></article>`);
}

// render the cards from the data and template
const cards = document.querySelector('#cards');
cards.innerHTML = diary_entrie().join('');
