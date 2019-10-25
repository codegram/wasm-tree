<script>
  import GameComponent from "./Game";

  let ready = false;
  let leaves;
  let trees;
  let fruits;

  import("../pkg").then(({ Leaves, Trees, Fruits }) => {
    leaves = Leaves.new();
    trees = Trees.new();
    fruits = Fruits.new();
    ready = true;
  });
</script>

{#if ready}
  <GameComponent {leaves} {trees} {fruits} />
{/if}
