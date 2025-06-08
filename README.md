## dominion_rust

A Rust implementation of the Dominion card game. Designed to be played with as a text interface.

This is part of a larger project that includes teaching an LLM to play Dominion. The goal is to create a system that can learn and adapt strategies based on game outcomes.

### How to play

Just clone the repo and use cargo to start a game:

```
cargo r
```

### Example game state

```
Current player: 0
Actions: 1
Buys: 1
Coins: 0
Current phase: TreasurePhase
Supply: Supply {
    treasures: {
        "Gold": 30,
        "Silver": 40,
        "Copper": 60,
    },
    actions: {
        "Moat": 10,
        "Smithy": 10,
        "Market": 10,
        "Laboratory": 10,
        "Festival": 10,
        "Village": 10,
    },
    victories: {
        "Province": 10,
        "Duchy": 10,
        "Estate": 10,
    },
    curses: {
        "Curse": 10,
    },
}
Current player deck: [
    Copper,
    Copper,
    Estate,
    Copper,
    Copper,
]
Current player discard: []
Current player hand: [
    Estate,
    Estate,
    Copper,
    Copper,
    Copper,
]
Current player played cards: []
>
```

### Moves

Available commands:

- play <card_index> - Play a card from your hand
- buy <card_name> - Buy a card from the supply
- end actions - End actions
- end treasures - End treasures
- end turn - End your turn
- help - Show this help message
- quit - Exit the game
