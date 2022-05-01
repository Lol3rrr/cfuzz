<script lang="ts">
    export let collapsed: boolean;

    let content_container;

    $: {
        swap(collapsed);
    }

    export function swap(_) {
        if (content_container === undefined) {
            return;
        }

        if (content_container.clientHeight) {
            content_container.style.height = 0;
        } else {
            content_container.style.height =
                content_container.scrollHeight + "px";
        }
    }
</script>

<div bind:this={content_container} class="expandable">
    <slot />
</div>

<style>
    .expandable {
        -moz-transition: height 0.5s;
        -ms-transition: height 0.5s;
        -o-transition: height 0.5s;
        -webkit-transition: height 0.5s;
        transition: height 0.5s;
        height: 0;
        overflow: hidden;
    }
</style>
