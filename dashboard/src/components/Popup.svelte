<script lang="ts">
</script>

<div class="popup">
    <slot />
</div>

<style>
    .popup {
        display: inline-block;
        position: absolute;
        width: 48vw;
        max-height: 80vh;
        padding: 1vh 1vw;

        top: 10vh;
        left: calc(20vw + ((80vw - 50vw) / 2));

        margin: 0px auto;

        background-color: #acacac;
        border-radius: 10px;

        z-index: 10;
    }
</style>
