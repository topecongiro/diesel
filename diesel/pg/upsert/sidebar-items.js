initSidebarItems({"fn":[["do_nothing","Used in conjunction with `on_conflict` to write a query in the form `ON CONFLICT (name) DO NOTHING`. If you want to do nothing when any constraint conflicts, use `on_conflict_do_nothing()` instead."],["do_update","Used to create a query in the form `ON CONFLICT (...) DO UPDATE ...`"],["excluded","Represents `excluded.column` in an `ON CONFLICT DO UPDATE` clause."],["on_constraint","Used to specify the constraint name for an upsert statement in the form `ON CONFLICT ON CONSTRAINT`. Note that `constraint_name` must be the name of a unique constraint, not the name of an index."]],"trait":[["OnConflictExtension","Adds extension methods related to PG upsert"]]});