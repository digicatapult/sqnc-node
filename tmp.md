### IN-93 - Token Finale or Sub Chaining

The reasoning behind this is mainly the performance and ability to group block by the given name. At the moment we are pulling one by one tokens from the very begining of the chain all the way to the last one. We also don't have the luxury to have two different envs e.g: `demo1` and `demo2`.

Events flowo in the nutshell
- user triggers an action 'reset' with the payload containing `{ name: "sub_chain_doe", ...  }`
    - rust node performs some basic validation e.g. if this name not being used and if needed more
    - creates a couple of tokens
        - *closing* - marks the end position of the chain and disables the last `opening`. This allows us to get the posisitions of `<name>` sub chain
        - *opening* - this token is being used as a ref for all others so they can be easily identified. It will contain some metadata such as `{ name: '', date: '', start: ''}`. This token_id should be included for every token as a `subchain_id` property. This way we can get the latest token and find `start` reference from which we will start pulling data (tokens).
    - when client tries to connect again and during the `initState` it should read the latest token and get the `subchain_id` which will allows us to identify a start point
        - all tokens/blocks should be pulled from node and stored in localstorage for better client performance and feel along with the latest_token. It should look like: `{ subchain_doe: { last_token: some_ref, data: [...items] }}`. This will allow us to fetch just the latest data as we know when was the last token. Some notes
            - this will require to do some basic validation to confirm that `opening` token is still active whiich can be done by telling client to check if latest token belongs to the same subchain -> `subchain_doe` and if not start the process again -> get subchain along with position and start fetching data.


### Changes in the nutshell
#### Substrate API 
This will require a couple of new endpoints
- ENDPOINT 1 - for getting `opening` token by the given id which will contain metadata, if still active only `start` and if not should have `start` and `end` refs along with other props
- ENDPOINT 2 - for getting all `subchain` this will allow us to switch between environments, but this might be out of scope as it adds complexity if we want to be able to mark it as active. Data will no longer be linear and this might require some expensive operations for stiching data together
- ENDPOINT 3 - for creating new tokens

#### Client Side
Not many changes here as form would be outside of the scope
- will need a middleware so when client performs an action it should check if there is nothing to fetch by using `last_token`
- middleware for storing tokens in localstorage (ideally whole store could be cached) in the following format: `{ sub_doe: { active: false, data: [], last_token: '' }, sub_lucy: { active: true, data: [], last_token: '' }}`
- potentially a new tab for the owner/master where it can call the above endpoints

#### Chain
- each token should include new property which is strongly typed `subchain_token_id` or something like that

The above reqs are very low level and logic is to be defined once we have agreed on the approach. Some outstanding questions:
- does closed subchain needs to be re-actived so new data can be pushed?
- where should we focus 
a: reducing the number of request and improving performance from client side
b: making blockchain as light as possible and increasing performance of our nodes
- do we want to allow client side to create a final token switch between subchains? (personally i prefer this option over the script or manual input)
- are chain api endpoints protected, e.g. auth token, can we distinguish a master/owner?

# TODO
- [ ] - draw some diagrams
- [ ] - read through and update where needed
- [ ] - think of all possible unknowns
- [ ] - review the implementation once agreed (chat with Matt and Jonathan)
- [ ] - 