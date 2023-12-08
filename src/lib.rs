mod types;

pub use types::*;

mod hcons;

// TODO: Finish querying logic
//       Need a one-line way to fetch collection, read entity, pass to callback

// TODO: Investigate structs as queries
//       Ideal would be to coerce from database into by-value, ref, mutable ref, option, etc by type
//       Seems inevitable that structs-as-inserts as structs-as-queries will diverge
//          Can create mappings between HLists of types using recursive traits, but no clear use case yet

// NOTE: Transpose - going from X of Ys to Y of Xs (i.e. a DB query going from collections of fields to collections of structs composed of a subset of those fields)

#[cfg(test)]
mod database_test {
    use crate::hcons::{
        CollectionContainer, ComponentDatabase, GetCollectionEntity, InsertGeneric, RemoveKey,
        RwLockRead, SculptView, ToDeref,
    };

    use frunk::{Generic, Hlist, LabelledGeneric, ToRef};

    type EntityKey = usize;
    type BoolDatabase = ComponentDatabase<EntityKey, bool>;
    type IntDatabase = ComponentDatabase<EntityKey, i32>;
    type FloatDatabase = ComponentDatabase<EntityKey, f32>;
    type StringDatabase = ComponentDatabase<EntityKey, &'static str>;

    type Database = Hlist![BoolDatabase, IntDatabase, FloatDatabase, StringDatabase,];

    type DatabaseHandle<'a> = <<Database as ToRef<'a>>::Output as ToDeref>::Output;

    #[derive(Debug, Copy, Clone, Generic, LabelledGeneric)]
    struct BoolIntFloatStringStruct {
        bool: bool,
        int: i32,
        float: f32,
        string: &'static str,
    }

    #[derive(Debug, Copy, Clone, Generic, LabelledGeneric)]
    struct IntFloatStruct {
        int: i32,
        float: f32,
    }

    #[derive(Debug, Copy, Clone, Generic, LabelledGeneric)]
    struct FloatStruct {
        float: f32,
    }

    #[test]
    fn typed_database() {
        let database: Database = Default::default();
        let database_ref = database.to_ref();
        let database_handle = database_ref.to_deref();

        populate_database(database_handle);
        read_database(database_handle);
    }

    fn populate_database(db: DatabaseHandle) {
        let bool_int_float_string_view = db.sculpt_view::<BoolIntFloatStringStruct>();

        bool_int_float_string_view.insert_generic(
            0,
            BoolIntFloatStringStruct {
                bool: false,
                int: 0,
                float: 0.0,
                string: "hey alright",
            },
        );

        bool_int_float_string_view.insert_generic(
            1,
            BoolIntFloatStringStruct {
                bool: true,
                int: 1,
                float: 0.2,
                string: "happy friday",
            },
        );

        /*
        bool_int_float_string_view.write_generic(1, |view: &mut BoolIntFloatStringStruct| {
            view.string = "happy big friday";
        });

        bool_int_float_string_view.read_generic(1, |view: &BoolIntFloatStringStruct| {
            println!("View: {:?}", view)
        });
        */

        let int_float_view = db.sculpt_view::<IntFloatStruct>();
        int_float_view.insert_generic(2, IntFloatStruct { int: 2, float: 0.4 });
        int_float_view.insert_generic(2, IntFloatStruct { int: 3, float: 0.6 });

        let float_view = db.sculpt_view::<FloatStruct>();
        float_view.insert_generic(4, FloatStruct { float: 0.8 });
        float_view.insert_generic(5, FloatStruct { float: 1.0 });

        db.remove_key(&4);
    }

    fn read_database(db: DatabaseHandle) {
        let int_float_view: Hlist![&CollectionContainer<usize, i32>, &CollectionContainer<usize, f32>] =
            db.sculpt_view::<IntFloatStruct>();

        let entity = int_float_view.get_collection_entity(&1);
        let entity = entity.to_ref();
        let entity = entity.to_deref();
        let entity = entity.read();
        let entity = entity.to_ref();
        let entity = entity.to_deref();
        println!("Entity 1: {:#?}", entity);

        /*
        let hlist: Hlist![i32, f32] = int_float_view.get_entity_hlist(&1);
        println!("HList: {:#?}", hlist);

        int_float_view.map_collection(|collection| {
            /*
            collection.map_entity(&1, |entity| {
                println!("Mapped Entity: {:#?}", entity);
            });
            */
        });

        //MapEntityHList::map_entity_hlist(int_float_view, &1, |hlist: Hlist![&i32, &f32]| println!("Mapped HList: {:#?}", hlist));

        let int_float_struct: IntFloatStruct = int_float_view.get_entity_generic(&1);
        println!("Struct: {:#?}", int_float_struct);
        */
    }
}
