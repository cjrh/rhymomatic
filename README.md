# rhymomatic

## Demo

Imagine you want to find rhymes for the word `twitch`:

```shell
$ ./target/release/rhymomatic -w twitch | column
bewitch		fritsch		krych		pitch		triche		wich
bewitch		fritsche	lich		pitsch		tritch		wich
bitch		fritzsche(2)	mich		rich		tritsch		witch
blitch		glitch		mitch		riche		twitch		witch
britsch		hitch		mitsch		ritch		twitch		zich
ditch		ich		niche(1)	snitch		unhitch
enrich		itch		nitsch		stich		which
enrich(1)	kitch		nitsche		stitch		which
fitch		kitsch		nycz		switch		which(1)
fritch		klich		piche		switch		which(1)
```

### vowel-matching

You could also find rhymes only on the vowel parts of `twitch`:

```shell
$ ./target/release/rhymomatic -w twitch -s vowel -m 3 | column
'til		dear		gwynne		menear		rid		therein
'tis		dechine(1)	gym		mere		ridge		thick
abridge		deer		gyn		mib		riff		thill
abril		deere		gyp		mic		riffe		thin
abyss		demille(1)	handpick	mich		rig		thing
acquit		demisch		hasid		mick		rigg		this
ad-lib		desir		herein		micke		rihn		this'
<snip many other hits>
```

This produces many more results, because there are many more matches. Would
you consider `gym` in the results above, to rhyme with `twitch`? Maybe not,
but the inner vowel part of the words is the same, and so even though
such a pairing is not a perfect rhyme, they match enough to be able to, 
for example, sing them in a rhyming pattern. 

### consonant-matching

We can also find rhymes that match only on the consonant parts:

```shell
$ ./target/release/rhymomatic -w twitch -s consonant -m 3 | column
IH-1		colorwatch	klanwatch	sandwich	twitch		wich
balcerowicz	corporatewatch	kolowich	sandwich(1)	wach		wiech
baywatch	creditwatch	krulwich	sandwich(2)	watch		witch
bewitch		currencywatch	moneywatch	stopwatch	watch(1)	wristwatch
bogdanowicz	deathwatch	norwich		swatch		weech		wyche
bromwich	dulwich		prestwich	swiech		which
butkiewicz	greenwich(1)	quach		switch		which(1)
```

Consider the pairing of `twitch` and `watch`: while not perfect rhymes, they
are certainly close enough to use then in many situations, again such as
song lyrics.

### Alliteration

Finally, consider searching for rhyming words that match _from the start_
rather than at the end. This is like alliteration:

```shell
$ ./target/release/rhymomatic -w twitch -t alliteration | column
'twas			tweezerman		twiddling		twinge			twins'			twisty
trois			tweezers		twiddy			twinge			twinsburg		twisty
tuolumne		twelfth			twiddy			twining			twinsburg		twitch
tuomi			twelve			twiford			twinjet			twirl			twitch
twaddell		twelvth			twiford			twinjet			twirled			twitched
twaddle			twenties		twig			twinjets		twirler			twitched
twain			twentieth		twig			twinjets		twirling		twitchell
twain's			twentieth(1)		twigg			twinkie			twirls			twitchell
twang			twenty			twigg			twinkie			twiss			twitches
twangy			twenty's		twigged			twinkies		twiss			twitches
twardowski		twenty-first		twigged			twinkies		twist			twitching
twardy			twenty-five		twiggs			twinkle			twist			twitching
twarog			twenty-four		twiggs			twinkle			twisted			twite
<snip>
```

An example one might use from these results could be something like
"twitchy twig" or something similar.

## CLI

This is what is printed out with the `-h` or `--help` parameter:

```shell
$ rhymomatic -h
rhymomatic x.y.z

USAGE:
    rhymomatic [FLAGS] [OPTIONS] --word <word>

FLAGS:
    -h, --help       Prints help information
    -n, --noemph     This setting will disable the requirement to match the emphasis in the given word
    -V, --version    Prints version information

OPTIONS:
    -m, --minphonemes <min-phonemes>    The minimum number of phonemes to match. The lower this is, the more matching
                                        words will be found, but the strength of the rhyme gets weaker. For example,
                                        with a min length of 1, the words "SANDALS" and "HIPPOS" will be matched because
                                        they share a single matching phoneme in the trailing "S" sound. Usually this is
                                        not what you want. A min length of 2-3 is recommended [default: 2]
    -s, --style <rhyme-style>           The style of rhyming. "syllabic" means to match both vowel and consonant sounds.
                                        "vowel" means to match only vowel sounds with consonants allowed to not match
                                        those in the given word. "consonant" is the opposite: only consonants in the
                                        given word will be matched, with vowels being allowed to be different [default:
                                        syllabic]
    -t, --type <rhyme-type>             The type of rhyme. "rhyme" means to try to match the given word from the end,
                                        like "POCUS" and "FOCUS". Alternatively you can give "alliteration", which will
                                        start matching from the front of the given word, like "POCUS" and "POCKET".
                                        Finally, you can provide "any", which means that phonemes in the given word will
                                        be allowed to match anywhere [default: rhyme]
    -w, --word <word>                   Provide the word to find rhymes for
```

## On Rhyme

This section is taken verbatim from [Rhyme](https://en.wikipedia.org/wiki/Rhyme):

The word rhyme can be used in a specific and a general sense. In the specific sense, two words rhyme if their final stressed vowel and all following sounds are identical; two lines of poetry rhyme if their final strong positions are filled with rhyming words. A rhyme in the strict sense is also called a perfect rhyme. Examples are sight and flight, deign and gain, madness and sadness, love and dove.
Perfect rhymes
Main article: Perfect rhyme

Perfect rhymes can be classified by the location of the final stressed syllable.

    single, also known as masculine: a rhyme in which the stress is on the final syllable of the words (rhyme, sublime)
    double, also known as feminine: a rhyme in which the stress is on the penultimate (second from last) syllable of the words (picky, tricky)
    dactylic: a rhyme in which the stress is on the antepenultimate (third from last) syllable (amorous, glamorous)

Feminine and dactylic rhymes may also be realized as compound (or mosaic) rhymes (poet, know it).
General rhymes

In the general sense, general rhyme can refer to various kinds of phonetic similarity between words, and to the use of such similar-sounding words in organizing verse. Rhymes in this general sense are classified according to the degree and manner of the phonetic similarity:

    syllabic: a rhyme in which the last syllable of each word sounds the same but does not necessarily contain stressed vowels. (cleaver, silver, or pitter, patter; the final syllable of the words bottle and fiddle is /l/, a liquid consonant.)
    imperfect (or near): a rhyme between a stressed and an unstressed syllable. (wing, caring)
    weak (or unaccented): a rhyme between two sets of one or more unstressed syllables. (hammer, carpenter)
    semirhyme: a rhyme with an extra syllable on one word. (bend, ending)
    forced (or oblique): a rhyme with an imperfect match in sound. (green, fiend; one, thumb)
    assonance: matching vowels. (shake, hate) Assonance is sometimes referred to as slant rhymes, along with consonance.
    consonance: matching consonants. (rabies, robbers)
    half rhyme (or slant rhyme): matching final consonants. (hand , lend)
    pararhyme: all consonants match. (tick, tock)
    alliteration (or head rhyme): matching initial consonants. (ship, short)
