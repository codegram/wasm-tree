<script>
  export let trees;
  export let leaves;
  export let fruits;

  let start = 0;
  let babies = [];

  const STATUS = {
    0: "DIED",
    1: "GROWN",
    2: "GROWING"
  };

  const updateState = () => {
    // sad
    leaves = leaves;
    trees = trees;
    fruits = fruits;
  };

  const handleTime = timestamp => {
    const elapsedTime = timestamp - start;
    start = timestamp;

    trees.create_leaves(leaves, elapsedTime / 1000);

    babies.forEach((baby, i) => {
      let grown = baby.grow(elapsedTime, trees);

      if (STATUS[grown] !== "GROWING") {
        babies.splice(i, 1);
      }
      if (STATUS[grown] === "DIED") {
        alert("the baby tree has died :(");
      }
    });
    updateState();

    window.requestAnimationFrame(handleTime);
  };

  window.requestAnimationFrame(handleTime);

  const handleCreateFruit = () => {
    fruits.add_fruit(leaves);
    updateState();
  };

  const handlePlantTree = () => {
    babies.push(trees.plant_tree(fruits));
    updateState();
  };
</script>

<h4>🍃 {Math.floor(leaves.count)} leaves</h4>

<h4>{trees.count} trees</h4>
<div>
  {#each new Array(trees.count) as tree}
    <span>🌳</span>
  {/each}
  {#each new Array(trees.baby_count) as tree}
    <span>🌱</span>
  {/each}
</div>
<h4>{fruits.count} fruits</h4>
<div>
  {#each new Array(fruits.count) as fruit}
    <span>🍎</span>
  {/each}
</div>
<hr />
<button
  on:click={handleCreateFruit}
  disabled={leaves.count < fruits.leaves_cost}>
  Create fruit ({fruits.leaves_cost} leaves)
</button>
<button on:click={handlePlantTree} disabled={fruits.count < 1}>
  Plant tree (1 fruit)
</button>
