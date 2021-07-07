#!/bin/sh
# SCRIPT: run.sh
# OPTIONS:
# IDENTITY (optional): custom/null/alice/bob/charlie/dave/eve/ferdie
# VALIDATOR (optional): null/true
# CHAIN (optional): null/chain_name
# BOOTNODES (optional): null/NODE_URL,NODE_URL2/etc
# WS,RPC,CORS (optional): null/true, null/true, null/all
# NODEKEY (optional): null/key

if [ "$IDENTITY" == dev ] || \
[ "$IDENTITY" == alice ] || \
[ "$IDENTITY" == bob ] || \
[ "$IDENTITY" == charlie ] || \
[ "$IDENTITY" == dave ] || \
[ "$IDENTITY" == eve ] || \
[ "$IDENTITY" == ferdie ]; then
	IDENTITYSHORTCUT="--${IDENTITY}";
else
	if [ -z "$IDENTITY" ]; then IDENTITY="test"; fi;
	IDENTITYSHORTCUT="--name ${IDENTITY}";
fi;

PORT="--port 30333";
RPCPORT="--ws-port 9944";
WSPORT="--rpc-port 9933";

if [ "$VALIDATOR" == true ]; then VALIDATOR="--validator"; fi;
if [ -z "$VALIDATOR" ]; then VALIDATOR=""; fi;

if [ "$IDENTITY" != dev ]; then
	if [ ! -z "$CHAIN" ]; then CHAIN="--chain ${CHAIN}"; fi;
	if [ -z "$CHAIN" ]; then
		CHAIN="--chain local";
	fi;
else
	CHAIN="";
fi;

if [ -z "$BOOTNODES" ]; then BOOTNODES=""; fi;
if [ ! -z "$BOOTNODES" ]; then
	BOOTNODESARR=$(echo $BOOTNODES|tr ", " "\\n");
	BOOTNODES="";
	for B in $BOOTNODESARR; do BOOTNODES="${BOOTNODES} --bootnodes ${B}"; done;
fi;

if [ "$WS" == true ]; then WS="--unsafe-ws-external"; else WS=""; fi;
if [ "$RPC" == true ]; then RPC="--unsafe-rpc-external"; else RPC=""; fi;
if [ "$CORS" == all ]; then CORS="--rpc-cors all"; else CORS=""; fi;

if [ ! -z "$NODEKEY" ]; then NODEKEY="--node-key ${NODEKEY}"; fi;
if [ -z "$NODEKEY" ]; then NODEKEY=""; fi;

./vitalam-node \
	--base-path /data/ \
	$IDENTITYSHORTCUT \
	$PORT $RPCPORT $WSPORT \
	$VALIDATOR \
	$CHAIN $BOOTNODES \
	$WS $RPC $CORS $NODEKEY
