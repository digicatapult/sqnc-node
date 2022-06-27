# Governance

This contains information about some of the governance procedures that take place in the Node. Initially this covers the voting process

## Create a Vote

In order to create a vote, in the Polkadot JS Apps frontend select the Governance tab, then `Tech. Comm`. You will then be presented with a screen with the current Governance members.

In order to start a new vote select proposals.

<img width="1680" alt="Screenshot 2022-06-27 at 14 02 49" src="../assets/1.png">

On the proposal screen select submit proposal on the right hand side. This will bring up a new proposal which must be made from an initial account, for example, here Alice is creating a proposal. You may notice the use of the doAs pallet, for more information visit [the readme](README.md) on how it works.

<img width="1278" alt="3" src="https://user-images.githubusercontent.com/35331926/175949271-6d9b543a-95aa-4a49-b637-dd1668c179af.png">

Once the proposal has been agreed and submit has been clicked another window will appear to confirm the transaction.

<img width="1083" alt="4" src="https://user-images.githubusercontent.com/35331926/175949544-3c542242-46e5-4718-a242-babb19ef2fe2.png">

Click sign and submit. There wiill then be a voting round.

<img width="1666" alt="Screenshot 2022-06-27 at 14 13 34" src="https://user-images.githubusercontent.com/35331926/175950428-ac60c688-8448-4729-b770-bda0a0c082a2.png">

<img width="720" alt="Screenshot 2022-06-27 at 14 14 08" src="https://user-images.githubusercontent.com/35331926/175950487-0765eb54-d8a0-4fe6-adbb-9dd62653b39a.png">

Once the voting round has been completed the proposal needs to be closed to enter the chain.

<img width="1680" alt="Screenshot 2022-06-27 at 14 14 33" src="https://user-images.githubusercontent.com/35331926/175950700-bb97d448-e2e4-42fc-85e6-2531c8aeb311.png">

You should see these for a successful vote.

<img width="332" alt="6" src="https://user-images.githubusercontent.com/35331926/175950857-2ee95f1d-c74e-4a2c-aa00-84cfd360d63b.png">

## Set Balance to 0

As we can see in the image below, Bob has nearly the same balance as Alice.

<img width="1676" alt="1" src="https://user-images.githubusercontent.com/35331926/175948878-3230e966-6361-49f1-b6f5-a4f268965cd7.png">

By following the voting rules above we can chance a users balance to 0.

<img width="1278" alt="3" src="https://user-images.githubusercontent.com/35331926/175950984-59593b7b-434d-42ab-8e2a-4460b000ca20.png">

<img width="1668" alt="7" src="https://user-images.githubusercontent.com/35331926/175951262-6eea7c14-018a-4df6-b123-844aad59e046.png">

## Can no longer transact

Now Bob has a balance of 0, if that party tries to create a proposal it will fail.

![8](https://user-images.githubusercontent.com/35331926/175951995-f4e1150e-5c15-45c1-ade6-1d6adcc99e47.png)

<img width="339" alt="9" src="https://user-images.githubusercontent.com/35331926/175952030-754ef849-a1fe-4283-b1e1-019219093fe3.png">
