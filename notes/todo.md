Items to be completed
=====================
- √ item pickup
- √ inventory
- √ dropping items
- √ bundle stacking
- √ equipping items
- √ unequpping items
- inventory capacity by weight
- tooltips
- √ first facility
- √ use of facility
- √ endorsements/enablements
- quests
- npc characters
- npc AIs
- attacking
- RT action
- √ chests
- √ facing

Faciltiies
===

Apple Tree:
===
#r 7,7 "An old apple tree" {
  property: apples => 35
  property: wood => 1000
  propery: age => 120     // in months

  future???
  timer(90000): increment(apples)
  timer(180000): increment(wood)
  timer(864_000_000): increment(age)
                      decrement(property)
                      toggle(property)
                      set(property)
                      clear(property)
                      
                      property => 1234
                      add(property,1234)
                      subtract(property,1234)
}


---
* Properties:
     logs left
     Apples left

* Pick apple
  * :can_pick  ?? should this be :can_pick??
  * equipment requirements: basket
  
* Chop tree
  
Lumber mill
---
* Mill logs


Fruit Press
---
* Press apples
* Press grapes
* Press olives

Chest/Locked Chest/Open Chest
---
* Unlock chest - use locked chest with key in inventory.
* Lock chest   - sneak use unlocked chest ???
* Open chest   - use unlocked chest
* Close chest  - move away from opened chest ?
* Installing lock - equip lock and use chest. 200 second timer.
* Removing lock - key must be in inventory. :can-remove-lock tool equipped.
                  100 second timer.  sneak-use unlocked chest.
* Lock picking  - :can-pick-lock tool equipped. can fail.  use locked chest.  must not have key
                  in inventory 100 second timer.  chance to reveal key.

