<div class="fill text-white bg-dark">
    <span class=""><h1>Sidebar</h1></span>
    <hr>
    <hr>
    <p>
        <a class="btn btn-primary" data-bs-toggle="collapse" href="#collapseExample" role="button" aria-expanded="false" aria-controls="collapseExample">
            Link with href
        </a>
        <button class="btn btn-primary" type="button" data-bs-toggle="collapse" data-bs-target="#collapseExample" aria-expanded="false" aria-controls="collapseExample">
            Button with data-bs-target
        </button>
    </p>
    <div class="collapse" id="collapseExample">
        <div class="card card-body">
            Some placeholder content for the collapse component. This panel is hidden by default but revealed when the user activates the relevant trigger.
        </div>
    </div>
</div>



<script>
let el = {
    "Database_name": {
        "Schemas": {
            "Public": {
                "Functions": ["array", "of", "functions"],
                "Views": ["array", "of", "views"],
                "Sequences": ["array", "of", "sequences"],
                "Triggers": ["array", "of", "triggers"],
                "Tables": {
                    "table_name": {
                        "Columns": ["array", "of", "columns"],
                        "Constraints": ["array", "of", "constraints"],
                        "Triggers": ["array", "of", "triggers"]
                    },
                    "second_table_name": {
                        "Columns": ["array", "of", "columns"],
                        "Constraints": ["array", "of", "constraints"],
                        "Triggers": ["array", "of", "triggers"]
                    }
                }
            }
        }
    }
}
</script>

<style>
    .fill {
        width: 100%;
        height: 100%;
    }
</style>
