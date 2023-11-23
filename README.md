# Mk^
Internal library for Misskey backend

## modules
- aid
    - const `aidRegExp`
    - function `genAid`
    - function `parseAid`
- correct_filename
    - function `correctFilename`
- secure_rndstr
    - const `L_CHARS`
    - function `secureRndstr`
- acct
    - interface `Acct`
    - function `parse`
    - function `toString`
- nsfw
    - class `Nsfw`
    - function `detect_sensitivity`