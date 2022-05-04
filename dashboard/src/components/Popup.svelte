<div class="popup">
    <slot />
</div>

<style>
    .popup {
        display: inline-block;
        position: absolute;
        width: 50vw;
        height: 80vw;

        top: 5px;
        left: calc(20vw + ((80vw - 50vw) / 2));

        margin: 0px auto;
    }
</style>
