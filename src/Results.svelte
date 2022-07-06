<div>
    {#if displayType == DisplayType.None}
        <div> Nothing to display here </div>
    {/if}

    {#if displayType == DisplayType.Value}
        <table class="table table-dark table-striped table-hover table-bordered table-sm">
            <thead>
                <tr>
                    <th></th>
                    {#each columns as column}
                        <th scope="col">{column}</th>
                    {/each}
                </tr>
            </thead>
            <tbody>
                {#each displayContent as row, index}
                    <tr>
                        <th scope="row">
                            {index + 1}
                        </th>
                        {#each row as el}
                            <td>{el}</td>
                        {/each}
                    </tr>
                {/each}
            </tbody>
        </table>
        modified: { changes }
    {/if}

    {#if displayType == DisplayType.Count}
        <div> How many columns were updated? </div>
    {/if}

    {#if displayType == DisplayType.Bool}
        <div> Did the operation go well? </div>
    {/if}

    {#if displayType == DisplayType.Error}
        <div> Something went bad! </div>
    {/if}
</div>

<script lang="ts">
    export let queryResults = ''

    enum DisplayType {
        Value, Count, Bool, Error, None
    }

    let json: any = ''
    let displayContent = ''
    let columns: Array<String> = []
    let changes: number = 0
    let displayType: DisplayType = DisplayType.None;

    export function update() {
        console.log(queryResults)
        json = JSON.parse(queryResults)

        if (json.hasOwnProperty('Value')) {
            displayType = DisplayType.Value
            displayContent = json.Value.values
            columns = json.Value.columns
            changes = json.Value.changes
        }

        if (json.hasOwnProperty('Count')) {
            displayType = DisplayType.Count
            displayContent = ''
            columns = []
            changes = json.Count.changes
        }

        if (json.hasOwnProperty('Bool')) {
            displayType = DisplayType.Bool
            displayContent = 'true'
            columns = []
            changes = 0
        }
    }
</script>

<style>

</style>
