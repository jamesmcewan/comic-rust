# Comicvine list comics from a publisher

For some reason the comicvine website does not have a way to list all the comics from a publisher. This is a simple cli tool that does that.

Requires a comicvine api key. You can get one by creating an account at https://comicvine.gamespot.com/api/ then add the `COMICVINE_API_KEY` to a `.env` file in the root of the project.


## Usage

Get the puvlisher id from the comicvine website. For example, the publisher id for Titan Books is `4010-4212`.

```bash
#Remember  the quotes
cargo run -- "4010-4212"


# To save the output to a file
cargo run -- "4010-4212" > boom_studios.txt
```
