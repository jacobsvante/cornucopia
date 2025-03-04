// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "clone_composite")]
        pub struct CloneComposite {
            #[postgres(name = "first")]
            pub first: i32,
            #[postgres(name = "second")]
            pub second: String,
        }
        #[derive(Debug)]
        pub struct CloneCompositeBorrowed<'a> {
            pub first: i32,
            pub second: &'a str,
        }
        impl<'a> From<CloneCompositeBorrowed<'a>> for CloneComposite {
            fn from(CloneCompositeBorrowed { first, second }: CloneCompositeBorrowed<'a>) -> Self {
                Self {
                    first,
                    second: second.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CloneCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<CloneCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let first = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let second = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(CloneCompositeBorrowed { first, second })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "clone_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CloneCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CloneCompositeBorrowed { first, second } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "clone_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            "second" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "copy_composite")]
        pub struct CopyComposite {
            #[postgres(name = "first")]
            pub first: i32,
            #[postgres(name = "second")]
            pub second: f64,
        }
        impl<'a> postgres_types::ToSql for CopyComposite {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CopyComposite { first, second } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "first" => postgres_types::ToSql::to_sql(first, field.type_(), out),
                        "second" => postgres_types::ToSql::to_sql(second, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "copy_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "first" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            "second" => <f64 as postgres_types::ToSql>::accepts(f.type_()),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "domain_composite")]
        pub struct DomainComposite {
            #[postgres(name = "txt")]
            pub txt: String,
            #[postgres(name = "json")]
            pub json: serde_json::Value,
            #[postgres(name = "nb")]
            pub nb: i32,
            #[postgres(name = "arr")]
            pub arr: Vec<serde_json::Value>,
        }
        #[derive(Debug)]
        pub struct DomainCompositeBorrowed<'a> {
            pub txt: &'a str,
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub arr: cornucopia_async::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
        }
        impl<'a> From<DomainCompositeBorrowed<'a>> for DomainComposite {
            fn from(
                DomainCompositeBorrowed { txt, json, nb, arr }: DomainCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    txt: txt.into(),
                    json: serde_json::from_str(json.0.get()).unwrap(),
                    nb,
                    arr: arr
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for DomainCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<DomainCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let txt = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let json = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let nb = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let arr = postgres_types::private::read_value(fields[3].type_(), &mut out)?;
                Ok(DomainCompositeBorrowed { txt, json, nb, arr })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "domain_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct DomainCompositeParams<'a> {
            pub txt: &'a str,
            pub json: &'a serde_json::value::Value,
            pub nb: i32,
            pub arr: &'a [&'a serde_json::value::Value],
        }
        impl<'a> postgres_types::ToSql for DomainCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let DomainCompositeParams { txt, json, nb, arr } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "txt" => postgres_types::ToSql::to_sql(
                            &cornucopia_async::private::Domain(txt),
                            field.type_(),
                            out,
                        ),
                        "json" => postgres_types::ToSql::to_sql(
                            &cornucopia_async::private::Domain(json),
                            field.type_(),
                            out,
                        ),
                        "nb" => postgres_types::ToSql::to_sql(
                            &cornucopia_async::private::Domain(nb),
                            field.type_(),
                            out,
                        ),
                        "arr" => postgres_types::ToSql::to_sql(
                            &cornucopia_async::private::Domain(
                                &cornucopia_async::private::DomainArray(arr),
                            ),
                            field.type_(),
                            out,
                        ),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "domain_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 4 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name()
                {
                    "txt" => <cornucopia_async::private::Domain::<&'a str> as
                    postgres_types::ToSql>::accepts(f.type_()),"json" => <cornucopia_async::private::Domain::<&'a serde_json::value::Value> as
                    postgres_types::ToSql>::accepts(f.type_()),"nb" => <cornucopia_async::private::Domain::<i32> as
                    postgres_types::ToSql>::accepts(f.type_()),"arr" => <cornucopia_async::private::Domain::<cornucopia_async::private::DomainArray::<&'a serde_json::value::Value, &[&'a serde_json::value::Value]>> as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "named_composite")]
        pub struct NamedComposite {
            #[postgres(name = "wow")]
            pub wow: Option<String>,
            #[postgres(name = "such_cool")]
            pub such_cool: Option<i32>,
        }
        #[derive(Debug)]
        pub struct NamedCompositeBorrowed<'a> {
            pub wow: Option<&'a str>,
            pub such_cool: Option<i32>,
        }
        impl<'a> From<NamedCompositeBorrowed<'a>> for NamedComposite {
            fn from(NamedCompositeBorrowed { wow, such_cool }: NamedCompositeBorrowed<'a>) -> Self {
                Self {
                    wow: wow.map(|v| v.into()),
                    such_cool,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NamedCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NamedCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(NamedCompositeBorrowed { wow, such_cool })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "named_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for NamedCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NamedCompositeBorrowed { wow, such_cool } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                        "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "named_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "wow" => <&'a str as postgres_types::ToSql>::accepts(f.type_()),
                            "such_cool" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum EnumWithDot {
            variant_with_dot,
        }
        impl<'a> postgres_types::ToSql for EnumWithDot {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    EnumWithDot::variant_with_dot => "variant.with_dot",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "enum.with_dot" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 1 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "variant.with_dot" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for EnumWithDot {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<EnumWithDot, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "variant.with_dot" => Ok(EnumWithDot::variant_with_dot),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "enum.with_dot" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 1 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "variant.with_dot" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "named_composite.with_dot")]
        pub struct NamedCompositeWithDot {
            #[postgres(name = "this.is.inconceivable")]
            pub this_is_inconceivable: Option<super::public::EnumWithDot>,
        }
        impl<'a> postgres_types::ToSql for NamedCompositeWithDot {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NamedCompositeWithDot {
                    this_is_inconceivable,
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "this.is.inconceivable" => {
                            postgres_types::ToSql::to_sql(this_is_inconceivable, field.type_(), out)
                        }
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "named_composite.with_dot" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 1 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "this.is.inconceivable" => {
                                <super::public::EnumWithDot as postgres_types::ToSql>::accepts(
                                    f.type_(),
                                )
                            }
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "nullity_composite")]
        pub struct NullityComposite {
            #[postgres(name = "jsons")]
            pub jsons: Option<Vec<Option<serde_json::Value>>>,
            #[postgres(name = "id")]
            pub id: i32,
        }
        #[derive(Debug)]
        pub struct NullityCompositeBorrowed<'a> {
            pub jsons: Option<
                cornucopia_async::ArrayIterator<
                    'a,
                    Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
                >,
            >,
            pub id: i32,
        }
        impl<'a> From<NullityCompositeBorrowed<'a>> for NullityComposite {
            fn from(NullityCompositeBorrowed { jsons, id }: NullityCompositeBorrowed<'a>) -> Self {
                Self {
                    jsons: jsons.map(|v| {
                        v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                            .collect()
                    }),
                    id,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NullityCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NullityCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let jsons = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let id = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                Ok(NullityCompositeBorrowed { jsons, id })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "nullity_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct NullityCompositeParams<'a> {
            pub jsons: Option<&'a [Option<&'a serde_json::value::Value>]>,
            pub id: i32,
        }
        impl<'a> postgres_types::ToSql for NullityCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NullityCompositeParams { jsons, id } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "jsons" => postgres_types::ToSql::to_sql(jsons, field.type_(), out),
                        "id" => postgres_types::ToSql::to_sql(id, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "nullity_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 2 {
                            return false;
                        }
                        fields.iter().all(|f| {
                            match f.name()
                {
                    "jsons" => <&'a [&'a serde_json::value::Value] as
                    postgres_types::ToSql>::accepts(f.type_()),"id" => <i32 as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                }
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum SpongebobCharacter {
            Bob,
            Patrick,
            Squidward,
        }
        impl<'a> postgres_types::ToSql for SpongebobCharacter {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    SpongebobCharacter::Bob => "Bob",
                    SpongebobCharacter::Patrick => "Patrick",
                    SpongebobCharacter::Squidward => "Squidward",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "spongebob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Bob" => true,
                            "Patrick" => true,
                            "Squidward" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for SpongebobCharacter {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<SpongebobCharacter, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "Bob" => Ok(SpongebobCharacter::Bob),
                    "Patrick" => Ok(SpongebobCharacter::Patrick),
                    "Squidward" => Ok(SpongebobCharacter::Squidward),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "spongebob_character" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "Bob" => true,
                            "Patrick" => true,
                            "Squidward" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "custom_composite")]
        pub struct CustomComposite {
            #[postgres(name = "wow")]
            pub wow: String,
            #[postgres(name = "such_cool")]
            pub such_cool: i32,
            #[postgres(name = "nice")]
            pub nice: super::public::SpongebobCharacter,
        }
        #[derive(Debug)]
        pub struct CustomCompositeBorrowed<'a> {
            pub wow: &'a str,
            pub such_cool: i32,
            pub nice: super::public::SpongebobCharacter,
        }
        impl<'a> From<CustomCompositeBorrowed<'a>> for CustomComposite {
            fn from(
                CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                }: CustomCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    wow: wow.into(),
                    such_cool,
                    nice,
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for CustomCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<CustomCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let wow = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let such_cool = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let nice = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                Ok(CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "custom_composite" && ty.schema() == "public"
            }
        }
        impl<'a> postgres_types::ToSql for CustomCompositeBorrowed<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let CustomCompositeBorrowed {
                    wow,
                    such_cool,
                    nice,
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "wow" => postgres_types::ToSql::to_sql(wow, field.type_(), out),
                        "such_cool" => postgres_types::ToSql::to_sql(such_cool, field.type_(), out),
                        "nice" => postgres_types::ToSql::to_sql(nice, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "custom_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name()
                {
                    "wow" => <&'a str as
                    postgres_types::ToSql>::accepts(f.type_()),"such_cool" => <i32 as
                    postgres_types::ToSql>::accepts(f.type_()),"nice" => <super::public::SpongebobCharacter as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Clone, PartialEq)]
        #[postgres(name = "nightmare_composite")]
        pub struct NightmareComposite {
            #[postgres(name = "custom")]
            pub custom: Vec<super::public::CustomComposite>,
            #[postgres(name = "spongebob")]
            pub spongebob: Vec<super::public::SpongebobCharacter>,
            #[postgres(name = "domain")]
            pub domain: String,
        }
        #[derive(Debug)]
        pub struct NightmareCompositeBorrowed<'a> {
            pub custom:
                cornucopia_async::ArrayIterator<'a, super::public::CustomCompositeBorrowed<'a>>,
            pub spongebob: cornucopia_async::ArrayIterator<'a, super::public::SpongebobCharacter>,
            pub domain: &'a str,
        }
        impl<'a> From<NightmareCompositeBorrowed<'a>> for NightmareComposite {
            fn from(
                NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                    domain,
                }: NightmareCompositeBorrowed<'a>,
            ) -> Self {
                Self {
                    custom: custom.map(|v| v.into()).collect(),
                    spongebob: spongebob.map(|v| v).collect(),
                    domain: domain.into(),
                }
            }
        }
        impl<'a> postgres_types::FromSql<'a> for NightmareCompositeBorrowed<'a> {
            fn from_sql(
                ty: &postgres_types::Type,
                out: &'a [u8],
            ) -> Result<NightmareCompositeBorrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(std::convert::Into::into(format!(
                        "invalid field count: {} vs {}",
                        num_fields,
                        fields.len()
                    )));
                }
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let custom = postgres_types::private::read_value(fields[0].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let spongebob = postgres_types::private::read_value(fields[1].type_(), &mut out)?;
                let _oid = postgres_types::private::read_be_i32(&mut out)?;
                let domain = postgres_types::private::read_value(fields[2].type_(), &mut out)?;
                Ok(NightmareCompositeBorrowed {
                    custom,
                    spongebob,
                    domain,
                })
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "nightmare_composite" && ty.schema() == "public"
            }
        }
        #[derive(Debug)]
        pub struct NightmareCompositeParams<'a> {
            pub custom: &'a [super::public::CustomCompositeBorrowed<'a>],
            pub spongebob: &'a [super::public::SpongebobCharacter],
            pub domain: &'a str,
        }
        impl<'a> postgres_types::ToSql for NightmareCompositeParams<'a> {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let NightmareCompositeParams {
                    custom,
                    spongebob,
                    domain,
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "custom" => postgres_types::ToSql::to_sql(custom, field.type_(), out),
                        "spongebob" => postgres_types::ToSql::to_sql(spongebob, field.type_(), out),
                        "domain" => postgres_types::ToSql::to_sql(
                            &cornucopia_async::private::Domain(domain),
                            field.type_(),
                            out,
                        ),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "nightmare_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 3 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name()
                {
                    "custom" => <&'a [super::public::CustomCompositeBorrowed<'a>] as
                    postgres_types::ToSql>::accepts(f.type_()),"spongebob" => <&'a [super::public::SpongebobCharacter] as
                    postgres_types::ToSql>::accepts(f.type_()),"domain" => <cornucopia_async::private::Domain::<&'a str> as
                    postgres_types::ToSql>::accepts(f.type_()),_ => false,
                })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, postgres_types::FromSql, Copy, Clone, PartialEq)]
        #[postgres(name = "syntax_composite")]
        pub struct SyntaxComposite {
            #[postgres(name = "async")]
            pub r#async: i32,
        }
        impl<'a> postgres_types::ToSql for SyntaxComposite {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let SyntaxComposite { r#async } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        "async" => postgres_types::ToSql::to_sql(r#async, field.type_(), out),
                        _ => unreachable!(),
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "syntax_composite" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != 1 {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            "async" => <i32 as postgres_types::ToSql>::accepts(f.type_()),
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        #[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum SyntaxEnum {
            r#async,
            r#box,
            I_Love_Chocolate,
        }
        impl<'a> postgres_types::ToSql for SyntaxEnum {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    SyntaxEnum::r#async => "async",
                    SyntaxEnum::r#box => "box",
                    SyntaxEnum::I_Love_Chocolate => "I Love Chocolate",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "syntax_enum" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "async" => true,
                            "box" => true,
                            "I Love Chocolate" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for SyntaxEnum {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<SyntaxEnum, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "async" => Ok(SyntaxEnum::r#async),
                    "box" => Ok(SyntaxEnum::r#box),
                    "I Love Chocolate" => Ok(SyntaxEnum::I_Love_Chocolate),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "syntax_enum" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "async" => true,
                            "box" => true,
                            "I Love Chocolate" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod copy {
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(
                    &postgres::Row,
                )
                    -> super::super::super::types::public::CloneCompositeBorrowed,
                mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
                ) -> PublicCloneCompositeQuery<'a, C, R, N> {
                    PublicCloneCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct PublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::super::super::types::public::CopyComposite,
                mapper: fn(super::super::super::types::public::CopyComposite) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCopyCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CopyComposite) -> R,
                ) -> PublicCopyCompositeQuery<'a, C, R, N> {
                    PublicCopyCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn insert_clone() -> InsertCloneStmt {
                InsertCloneStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO clone (composite) VALUES ($1)",
                ))
            }
            pub struct InsertCloneStmt(cornucopia_sync::private::Stmt);
            impl InsertCloneStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    composite: &'a super::super::super::types::public::CloneCompositeBorrowed<'a>,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[composite])
                }
            }
            pub fn select_clone() -> SelectCloneStmt {
                SelectCloneStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM clone"))
            }
            pub struct SelectCloneStmt(cornucopia_sync::private::Stmt);
            impl SelectCloneStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn insert_copy() -> InsertCopyStmt {
                InsertCopyStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO copy (composite) VALUES ($1)",
                ))
            }
            pub struct InsertCopyStmt(cornucopia_sync::private::Stmt);
            impl InsertCopyStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    composite: &'a super::super::super::types::public::CopyComposite,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[composite])
                }
            }
            pub fn select_copy() -> SelectCopyStmt {
                SelectCopyStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM copy"))
            }
            pub struct SelectCopyStmt(cornucopia_sync::private::Stmt);
            impl SelectCopyStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> PublicCopyCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CopyComposite,
                    0,
                > {
                    PublicCopyCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(
                    &tokio_postgres::Row,
                )
                    -> super::super::super::types::public::CloneCompositeBorrowed,
                mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
                ) -> PublicCloneCompositeQuery<'a, C, R, N> {
                    PublicCloneCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct PublicCopyCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor:
                    fn(&tokio_postgres::Row) -> super::super::super::types::public::CopyComposite,
                mapper: fn(super::super::super::types::public::CopyComposite) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCopyCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CopyComposite) -> R,
                ) -> PublicCopyCompositeQuery<'a, C, R, N> {
                    PublicCopyCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn insert_clone() -> InsertCloneStmt {
                InsertCloneStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO clone (composite) VALUES ($1)",
                ))
            }
            pub struct InsertCloneStmt(cornucopia_async::private::Stmt);
            impl InsertCloneStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    composite: &'a super::super::super::types::public::CloneCompositeBorrowed<'a>,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[composite]).await
                }
            }
            pub fn select_clone() -> SelectCloneStmt {
                SelectCloneStmt(cornucopia_async::private::Stmt::new("SELECT * FROM clone"))
            }
            pub struct SelectCloneStmt(cornucopia_async::private::Stmt);
            impl SelectCloneStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn insert_copy() -> InsertCopyStmt {
                InsertCopyStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO copy (composite) VALUES ($1)",
                ))
            }
            pub struct InsertCopyStmt(cornucopia_async::private::Stmt);
            impl InsertCopyStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    composite: &'a super::super::super::types::public::CopyComposite,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[composite]).await
                }
            }
            pub fn select_copy() -> SelectCopyStmt {
                SelectCopyStmt(cornucopia_async::private::Stmt::new("SELECT * FROM copy"))
            }
            pub struct SelectCopyStmt(cornucopia_async::private::Stmt);
            impl SelectCopyStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> PublicCopyCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CopyComposite,
                    0,
                > {
                    PublicCopyCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
        }
    }
    pub mod domain {
        #[derive(Debug)]
        pub struct InsertNightmareDomainParams<
            'a,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::JsonSql,
            T3: cornucopia_async::JsonSql,
            T4: cornucopia_async::ArraySql<Item = T3>,
        > {
            pub txt: T1,
            pub json: T2,
            pub nb: i32,
            pub arr: T4,
            pub composite: Option<super::super::types::public::DomainCompositeParams<'a>>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomain {
            pub txt: String,
            pub json: serde_json::Value,
            pub nb: i32,
            pub arr: Vec<serde_json::Value>,
        }
        pub struct SelectNightmareDomainBorrowed<'a> {
            pub txt: &'a str,
            pub json: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub nb: i32,
            pub arr: cornucopia_async::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
        }
        impl<'a> From<SelectNightmareDomainBorrowed<'a>> for SelectNightmareDomain {
            fn from(
                SelectNightmareDomainBorrowed { txt, json, nb, arr }: SelectNightmareDomainBorrowed<
                    'a,
                >,
            ) -> Self {
                Self {
                    txt: txt.into(),
                    json: serde_json::from_str(json.0.get()).unwrap(),
                    nb,
                    arr: arr
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectNightmareDomainNull {
            pub txt: Option<String>,
            pub json: Option<serde_json::Value>,
            pub nb: Option<i32>,
            pub arr: Option<Vec<Option<serde_json::Value>>>,
            pub composite: Option<super::super::types::public::DomainComposite>,
        }
        pub struct SelectNightmareDomainNullBorrowed<'a> {
            pub txt: Option<&'a str>,
            pub json: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub nb: Option<i32>,
            pub arr: Option<
                cornucopia_async::ArrayIterator<
                    'a,
                    Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
                >,
            >,
            pub composite: Option<super::super::types::public::DomainCompositeBorrowed<'a>>,
        }
        impl<'a> From<SelectNightmareDomainNullBorrowed<'a>> for SelectNightmareDomainNull {
            fn from(
                SelectNightmareDomainNullBorrowed {
                    txt,
                    json,
                    nb,
                    arr,
                    composite,
                }: SelectNightmareDomainNullBorrowed<'a>,
            ) -> Self {
                Self {
                    txt: txt.map(|v| v.into()),
                    json: json.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    nb,
                    arr: arr.map(|v| {
                        v.map(|v| v.map(|v| serde_json::from_str(v.0.get()).unwrap()))
                            .collect()
                    }),
                    composite: composite.map(|v| v.into()),
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::SelectNightmareDomainBorrowed,
                mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
                ) -> SelectNightmareDomainQuery<'a, C, R, N> {
                    SelectNightmareDomainQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
                mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
                ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
                    SelectNightmareDomainNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
                SelectNightmareDomainStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT txt, json, nb, arr FROM nightmare_domain",
                ))
            }
            pub struct SelectNightmareDomainStmt(cornucopia_sync::private::Stmt);
            impl SelectNightmareDomainStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0>
                {
                    SelectNightmareDomainQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectNightmareDomainBorrowed {
                            txt: row.get(0),
                            json: row.get(1),
                            nb: row.get(2),
                            arr: row.get(3),
                        },
                        mapper: |it| <super::SelectNightmareDomain>::from(it),
                    }
                }
            }
            pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
                InsertNightmareDomainStmt(cornucopia_sync::private::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
            }
            pub struct InsertNightmareDomainStmt(cornucopia_sync::private::Stmt);
            impl InsertNightmareDomainStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::JsonSql,
                    T3: cornucopia_sync::JsonSql,
                    T4: cornucopia_sync::ArraySql<Item = T3>,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    txt: &'a T1,
                    json: &'a T2,
                    nb: &'a i32,
                    arr: &'a T4,
                    composite: &'a Option<
                        super::super::super::types::public::DomainCompositeParams<'a>,
                    >,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(
                        stmt,
                        &[
                            &cornucopia_sync::private::Domain(txt),
                            &cornucopia_sync::private::Domain(json),
                            &cornucopia_sync::private::Domain(nb),
                            &cornucopia_sync::private::Domain(
                                &cornucopia_sync::private::DomainArray(arr),
                            ),
                            composite,
                        ],
                    )
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::JsonSql,
                    T3: cornucopia_sync::JsonSql,
                    T4: cornucopia_sync::ArraySql<Item = T3>,
                >
                cornucopia_sync::Params<
                    'a,
                    super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
                    Result<u64, postgres::Error>,
                    C,
                > for InsertNightmareDomainStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(
                        client,
                        &params.txt,
                        &params.json,
                        &params.nb,
                        &params.arr,
                        &params.composite,
                    )
                }
            }
            pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
                SelectNightmareDomainNullStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT * FROM nightmare_domain",
                ))
            }
            pub struct SelectNightmareDomainNullStmt(cornucopia_sync::private::Stmt);
            impl SelectNightmareDomainNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0>
                {
                    SelectNightmareDomainNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectNightmareDomainNullBorrowed {
                            txt: row.get(0),
                            json: row.get(1),
                            nb: row.get(2),
                            arr: row.get(3),
                            composite: row.get(4),
                        },
                        mapper: |it| <super::SelectNightmareDomainNull>::from(it),
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct SelectNightmareDomainQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainBorrowed,
                mapper: fn(super::SelectNightmareDomainBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectNightmareDomainBorrowed) -> R,
                ) -> SelectNightmareDomainQuery<'a, C, R, N> {
                    SelectNightmareDomainQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct SelectNightmareDomainNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::SelectNightmareDomainNullBorrowed,
                mapper: fn(super::SelectNightmareDomainNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectNightmareDomainNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectNightmareDomainNullBorrowed) -> R,
                ) -> SelectNightmareDomainNullQuery<'a, C, R, N> {
                    SelectNightmareDomainNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn select_nightmare_domain() -> SelectNightmareDomainStmt {
                SelectNightmareDomainStmt(cornucopia_async::private::Stmt::new(
                    "SELECT txt, json, nb, arr FROM nightmare_domain",
                ))
            }
            pub struct SelectNightmareDomainStmt(cornucopia_async::private::Stmt);
            impl SelectNightmareDomainStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> SelectNightmareDomainQuery<'a, C, super::SelectNightmareDomain, 0>
                {
                    SelectNightmareDomainQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectNightmareDomainBorrowed {
                            txt: row.get(0),
                            json: row.get(1),
                            nb: row.get(2),
                            arr: row.get(3),
                        },
                        mapper: |it| <super::SelectNightmareDomain>::from(it),
                    }
                }
            }
            pub fn insert_nightmare_domain() -> InsertNightmareDomainStmt {
                InsertNightmareDomainStmt(cornucopia_async::private::Stmt::new("INSERT INTO nightmare_domain (txt, json, nb, arr, composite) VALUES ($1, $2, $3, $4, $5)"))
            }
            pub struct InsertNightmareDomainStmt(cornucopia_async::private::Stmt);
            impl InsertNightmareDomainStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::JsonSql,
                    T3: cornucopia_async::JsonSql,
                    T4: cornucopia_async::ArraySql<Item = T3>,
                >(
                    &'a mut self,
                    client: &'a C,
                    txt: &'a T1,
                    json: &'a T2,
                    nb: &'a i32,
                    arr: &'a T4,
                    composite: &'a Option<
                        super::super::super::types::public::DomainCompositeParams<'a>,
                    >,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client
                        .execute(
                            stmt,
                            &[
                                &cornucopia_async::private::Domain(txt),
                                &cornucopia_async::private::Domain(json),
                                &cornucopia_async::private::Domain(nb),
                                &cornucopia_async::private::Domain(
                                    &cornucopia_async::private::DomainArray(arr),
                                ),
                                composite,
                            ],
                        )
                        .await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::JsonSql,
                    T3: cornucopia_async::JsonSql,
                    T4: cornucopia_async::ArraySql<Item = T3>,
                >
                cornucopia_async::Params<
                    'a,
                    super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for InsertNightmareDomainStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::InsertNightmareDomainParams<'a, T1, T2, T3, T4>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(
                        client,
                        &params.txt,
                        &params.json,
                        &params.nb,
                        &params.arr,
                        &params.composite,
                    ))
                }
            }
            pub fn select_nightmare_domain_null() -> SelectNightmareDomainNullStmt {
                SelectNightmareDomainNullStmt(cornucopia_async::private::Stmt::new(
                    "SELECT * FROM nightmare_domain",
                ))
            }
            pub struct SelectNightmareDomainNullStmt(cornucopia_async::private::Stmt);
            impl SelectNightmareDomainNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> SelectNightmareDomainNullQuery<'a, C, super::SelectNightmareDomainNull, 0>
                {
                    SelectNightmareDomainNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectNightmareDomainNullBorrowed {
                            txt: row.get(0),
                            json: row.get(1),
                            nb: row.get(2),
                            arr: row.get(3),
                            composite: row.get(4),
                        },
                        mapper: |it| <super::SelectNightmareDomainNull>::from(it),
                    }
                }
            }
        }
    }
    pub mod named {
        #[derive(Debug)]
        pub struct NamedParams<T1: cornucopia_async::StringSql> {
            pub name: T1,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct NamedComplexParams<'a> {
            pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
            pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Id {
            pub id: i32,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Named {
            pub id: i32,
            pub name: String,
            pub price: Option<f64>,
            pub show: bool,
        }
        pub struct NamedBorrowed<'a> {
            pub id: i32,
            pub name: &'a str,
            pub price: Option<f64>,
            pub show: bool,
        }
        impl<'a> From<NamedBorrowed<'a>> for Named {
            fn from(
                NamedBorrowed {
                    id,
                    name,
                    price,
                    show,
                }: NamedBorrowed<'a>,
            ) -> Self {
                Self {
                    id,
                    name: name.into(),
                    price,
                    show,
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct NamedComplex {
            pub named: super::super::types::public::NamedComposite,
            pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
        }
        pub struct NamedComplexBorrowed<'a> {
            pub named: super::super::types::public::NamedCompositeBorrowed<'a>,
            pub named_with_dot: Option<super::super::types::public::NamedCompositeWithDot>,
        }
        impl<'a> From<NamedComplexBorrowed<'a>> for NamedComplex {
            fn from(
                NamedComplexBorrowed {
                    named,
                    named_with_dot,
                }: NamedComplexBorrowed<'a>,
            ) -> Self {
                Self {
                    named: named.into(),
                    named_with_dot,
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::Id,
                mapper: fn(super::Id) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
                    IdQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::NamedBorrowed,
                mapper: fn(super::NamedBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NamedBorrowed) -> R,
                ) -> NamedQuery<'a, C, R, N> {
                    NamedQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::NamedComplexBorrowed,
                mapper: fn(super::NamedComplexBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NamedComplexBorrowed) -> R,
                ) -> NamedComplexQuery<'a, C, R, N> {
                    NamedComplexQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn new_named_visible() -> NewNamedVisibleStmt {
                NewNamedVisibleStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
                ))
            }
            pub struct NewNamedVisibleStmt(cornucopia_sync::private::Stmt);
            impl NewNamedVisibleStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a T1,
                    price: &'a Option<f64>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    IdQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::Id { id: row.get(0) },
                        mapper: |it| <super::Id>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
                for NewNamedVisibleStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::NamedParams<T1>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn new_named_hidden() -> NewNamedHiddenStmt {
                NewNamedHiddenStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NewNamedHiddenStmt(cornucopia_sync::private::Stmt);
            impl NewNamedHiddenStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    price: &'a Option<f64>,
                    name: &'a T1,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    IdQuery {
                        client,
                        params: [price, name],
                        stmt: &mut self.0,
                        extractor: |row| super::Id { id: row.get(0) },
                        mapper: |it| <super::Id>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<'a, super::NamedParams<T1>, IdQuery<'a, C, super::Id, 2>, C>
                for NewNamedHiddenStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::NamedParams<T1>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    self.bind(client, &params.price, &params.name)
                }
            }
            pub fn named() -> NamedStmt {
                NamedStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM named"))
            }
            pub struct NamedStmt(cornucopia_sync::private::Stmt);
            impl NamedStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> NamedQuery<'a, C, super::Named, 0> {
                    NamedQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            price: row.get(2),
                            show: row.get(3),
                        },
                        mapper: |it| <super::Named>::from(it),
                    }
                }
            }
            pub fn named_by_id() -> NamedByIdStmt {
                NamedByIdStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT * FROM named WHERE id = $1",
                ))
            }
            pub struct NamedByIdStmt(cornucopia_sync::private::Stmt);
            impl NamedByIdStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    id: &'a i32,
                ) -> NamedQuery<'a, C, super::Named, 1> {
                    NamedQuery {
                        client,
                        params: [id],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            price: row.get(2),
                            show: row.get(3),
                        },
                        mapper: |it| <super::Named>::from(it),
                    }
                }
            }
            pub fn new_named_complex() -> NewNamedComplexStmt {
                NewNamedComplexStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
                ))
            }
            pub struct NewNamedComplexStmt(cornucopia_sync::private::Stmt);
            impl NewNamedComplexStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    named: &'a super::super::super::types::public::NamedCompositeBorrowed<'a>,
                    named_with_dot: &'a Option<
                        super::super::super::types::public::NamedCompositeWithDot,
                    >,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[named, named_with_dot])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::NamedComplexParams<'a>,
                    Result<u64, postgres::Error>,
                    C,
                > for NewNamedComplexStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::NamedComplexParams<'a>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.named, &params.named_with_dot)
                }
            }
            pub fn named_complex() -> NamedComplexStmt {
                NamedComplexStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT * FROM named_complex",
                ))
            }
            pub struct NamedComplexStmt(cornucopia_sync::private::Stmt);
            impl NamedComplexStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
                    NamedComplexQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedComplexBorrowed {
                            named: row.get(0),
                            named_with_dot: row.get(1),
                        },
                        mapper: |it| <super::NamedComplex>::from(it),
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct IdQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::Id,
                mapper: fn(super::Id) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> IdQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(super::Id) -> R) -> IdQuery<'a, C, R, N> {
                    IdQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct NamedQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::NamedBorrowed,
                mapper: fn(super::NamedBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NamedQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NamedBorrowed) -> R,
                ) -> NamedQuery<'a, C, R, N> {
                    NamedQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct NamedComplexQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::NamedComplexBorrowed,
                mapper: fn(super::NamedComplexBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NamedComplexQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NamedComplexBorrowed) -> R,
                ) -> NamedComplexQuery<'a, C, R, N> {
                    NamedComplexQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn new_named_visible() -> NewNamedVisibleStmt {
                NewNamedVisibleStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, true) RETURNING id ",
                ))
            }
            pub struct NewNamedVisibleStmt(cornucopia_async::private::Stmt);
            impl NewNamedVisibleStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a T1,
                    price: &'a Option<f64>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    IdQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::Id { id: row.get(0) },
                        mapper: |it| <super::Id>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::NamedParams<T1>,
                    IdQuery<'a, C, super::Id, 2>,
                    C,
                > for NewNamedVisibleStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::NamedParams<T1>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn new_named_hidden() -> NewNamedHiddenStmt {
                NewNamedHiddenStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (price, name, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NewNamedHiddenStmt(cornucopia_async::private::Stmt);
            impl NewNamedHiddenStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    price: &'a Option<f64>,
                    name: &'a T1,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    IdQuery {
                        client,
                        params: [price, name],
                        stmt: &mut self.0,
                        extractor: |row| super::Id { id: row.get(0) },
                        mapper: |it| <super::Id>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::NamedParams<T1>,
                    IdQuery<'a, C, super::Id, 2>,
                    C,
                > for NewNamedHiddenStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::NamedParams<T1>,
                ) -> IdQuery<'a, C, super::Id, 2> {
                    self.bind(client, &params.price, &params.name)
                }
            }
            pub fn named() -> NamedStmt {
                NamedStmt(cornucopia_async::private::Stmt::new("SELECT * FROM named"))
            }
            pub struct NamedStmt(cornucopia_async::private::Stmt);
            impl NamedStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> NamedQuery<'a, C, super::Named, 0> {
                    NamedQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            price: row.get(2),
                            show: row.get(3),
                        },
                        mapper: |it| <super::Named>::from(it),
                    }
                }
            }
            pub fn named_by_id() -> NamedByIdStmt {
                NamedByIdStmt(cornucopia_async::private::Stmt::new(
                    "SELECT * FROM named WHERE id = $1",
                ))
            }
            pub struct NamedByIdStmt(cornucopia_async::private::Stmt);
            impl NamedByIdStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    id: &'a i32,
                ) -> NamedQuery<'a, C, super::Named, 1> {
                    NamedQuery {
                        client,
                        params: [id],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedBorrowed {
                            id: row.get(0),
                            name: row.get(1),
                            price: row.get(2),
                            show: row.get(3),
                        },
                        mapper: |it| <super::Named>::from(it),
                    }
                }
            }
            pub fn new_named_complex() -> NewNamedComplexStmt {
                NewNamedComplexStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named_complex (named, \"named.with_dot\") VALUES ($1, $2)",
                ))
            }
            pub struct NewNamedComplexStmt(cornucopia_async::private::Stmt);
            impl NewNamedComplexStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    named: &'a super::super::super::types::public::NamedCompositeBorrowed<'a>,
                    named_with_dot: &'a Option<
                        super::super::super::types::public::NamedCompositeWithDot,
                    >,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[named, named_with_dot]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::NamedComplexParams<'a>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for NewNamedComplexStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::NamedComplexParams<'a>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.named, &params.named_with_dot))
                }
            }
            pub fn named_complex() -> NamedComplexStmt {
                NamedComplexStmt(cornucopia_async::private::Stmt::new(
                    "SELECT * FROM named_complex",
                ))
            }
            pub struct NamedComplexStmt(cornucopia_async::private::Stmt);
            impl NamedComplexStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> NamedComplexQuery<'a, C, super::NamedComplex, 0> {
                    NamedComplexQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NamedComplexBorrowed {
                            named: row.get(0),
                            named_with_dot: row.get(1),
                        },
                        mapper: |it| <super::NamedComplex>::from(it),
                    }
                }
            }
        }
    }
    pub mod nullity {
        #[derive(Debug)]
        pub struct NullityParams<
            'a,
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::ArraySql<Item = Option<T1>>,
            T3: cornucopia_async::StringSql,
        > {
            pub texts: T2,
            pub name: T3,
            pub composite: Option<super::super::types::public::NullityCompositeParams<'a>>,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Nullity {
            pub texts: Vec<Option<String>>,
            pub name: String,
            pub composite: Option<super::super::types::public::NullityComposite>,
        }
        pub struct NullityBorrowed<'a> {
            pub texts: cornucopia_async::ArrayIterator<'a, Option<&'a str>>,
            pub name: &'a str,
            pub composite: Option<super::super::types::public::NullityCompositeBorrowed<'a>>,
        }
        impl<'a> From<NullityBorrowed<'a>> for Nullity {
            fn from(
                NullityBorrowed {
                    texts,
                    name,
                    composite,
                }: NullityBorrowed<'a>,
            ) -> Self {
                Self {
                    texts: texts.map(|v| v.map(|v| v.into())).collect(),
                    name: name.into(),
                    composite: composite.map(|v| v.into()),
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::NullityBorrowed,
                mapper: fn(super::NullityBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NullityBorrowed) -> R,
                ) -> NullityQuery<'a, C, R, N> {
                    NullityQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn new_nullity() -> NewNullityStmt {
                NewNullityStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
                ))
            }
            pub struct NewNullityStmt(cornucopia_sync::private::Stmt);
            impl NewNullityStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::ArraySql<Item = Option<T1>>,
                    T3: cornucopia_sync::StringSql,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    texts: &'a T2,
                    name: &'a T3,
                    composite: &'a Option<
                        super::super::super::types::public::NullityCompositeParams<'a>,
                    >,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[texts, name, composite])
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::ArraySql<Item = Option<T1>>,
                    T3: cornucopia_sync::StringSql,
                >
                cornucopia_sync::Params<
                    'a,
                    super::NullityParams<'a, T1, T2, T3>,
                    Result<u64, postgres::Error>,
                    C,
                > for NewNullityStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::NullityParams<'a, T1, T2, T3>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.texts, &params.name, &params.composite)
                }
            }
            pub fn nullity() -> NullityStmt {
                NullityStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM nullity"))
            }
            pub struct NullityStmt(cornucopia_sync::private::Stmt);
            impl NullityStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> NullityQuery<'a, C, super::Nullity, 0> {
                    NullityQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NullityBorrowed {
                            texts: row.get(0),
                            name: row.get(1),
                            composite: row.get(2),
                        },
                        mapper: |it| <super::Nullity>::from(it),
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct NullityQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::NullityBorrowed,
                mapper: fn(super::NullityBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> NullityQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::NullityBorrowed) -> R,
                ) -> NullityQuery<'a, C, R, N> {
                    NullityQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn new_nullity() -> NewNullityStmt {
                NewNullityStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO nullity(texts, name, composite) VALUES ($1, $2, $3)",
                ))
            }
            pub struct NewNullityStmt(cornucopia_async::private::Stmt);
            impl NewNullityStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::ArraySql<Item = Option<T1>>,
                    T3: cornucopia_async::StringSql,
                >(
                    &'a mut self,
                    client: &'a C,
                    texts: &'a T2,
                    name: &'a T3,
                    composite: &'a Option<
                        super::super::super::types::public::NullityCompositeParams<'a>,
                    >,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[texts, name, composite]).await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::ArraySql<Item = Option<T1>>,
                    T3: cornucopia_async::StringSql,
                >
                cornucopia_async::Params<
                    'a,
                    super::NullityParams<'a, T1, T2, T3>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for NewNullityStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::NullityParams<'a, T1, T2, T3>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.texts, &params.name, &params.composite))
                }
            }
            pub fn nullity() -> NullityStmt {
                NullityStmt(cornucopia_async::private::Stmt::new(
                    "SELECT * FROM nullity",
                ))
            }
            pub struct NullityStmt(cornucopia_async::private::Stmt);
            impl NullityStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> NullityQuery<'a, C, super::Nullity, 0> {
                    NullityQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::NullityBorrowed {
                            texts: row.get(0),
                            name: row.get(1),
                            composite: row.get(2),
                        },
                        mapper: |it| <super::Nullity>::from(it),
                    }
                }
            }
        }
    }
    pub mod params {
        #[derive(Debug)]
        pub struct InsertBookParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub author: Option<T1>,
            pub name: T2,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct ParamsOrderParams {
            pub c: i32,
            pub a: i32,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct SelectBook {
            pub name: String,
            pub author: Option<String>,
        }
        pub struct SelectBookBorrowed<'a> {
            pub name: &'a str,
            pub author: Option<&'a str>,
        }
        impl<'a> From<SelectBookBorrowed<'a>> for SelectBook {
            fn from(SelectBookBorrowed { name, author }: SelectBookBorrowed<'a>) -> Self {
                Self {
                    name: name.into(),
                    author: author.map(|v| v.into()),
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct FindBooks {
            pub name: String,
            pub author: Option<String>,
        }
        pub struct FindBooksBorrowed<'a> {
            pub name: &'a str,
            pub author: Option<&'a str>,
        }
        impl<'a> From<FindBooksBorrowed<'a>> for FindBooks {
            fn from(FindBooksBorrowed { name, author }: FindBooksBorrowed<'a>) -> Self {
                Self {
                    name: name.into(),
                    author: author.map(|v| v.into()),
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::SelectBookBorrowed,
                mapper: fn(super::SelectBookBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectBookBorrowed) -> R,
                ) -> SelectBookQuery<'a, C, R, N> {
                    SelectBookQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::FindBooksBorrowed,
                mapper: fn(super::FindBooksBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::FindBooksBorrowed) -> R,
                ) -> FindBooksQuery<'a, C, R, N> {
                    FindBooksQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn insert_book() -> InsertBookStmt {
                InsertBookStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO book (author, name) VALUES ($1, $2)",
                ))
            }
            pub struct InsertBookStmt(cornucopia_sync::private::Stmt);
            impl InsertBookStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    author: &'a Option<T1>,
                    name: &'a T2,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[author, name])
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                >
                cornucopia_sync::Params<
                    'a,
                    super::InsertBookParams<T1, T2>,
                    Result<u64, postgres::Error>,
                    C,
                > for InsertBookStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::InsertBookParams<T1, T2>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.author, &params.name)
                }
            }
            pub fn select_book() -> SelectBookStmt {
                SelectBookStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM book"))
            }
            pub struct SelectBookStmt(cornucopia_sync::private::Stmt);
            impl SelectBookStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
                    SelectBookQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectBookBorrowed {
                            name: row.get(0),
                            author: row.get(1),
                        },
                        mapper: |it| <super::SelectBook>::from(it),
                    }
                }
            }
            pub fn find_books() -> FindBooksStmt {
                FindBooksStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT * FROM book WHERE name = ANY ($1)",
                ))
            }
            pub struct FindBooksStmt(cornucopia_sync::private::Stmt);
            impl FindBooksStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::ArraySql<Item = T1>,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    title: &'a T2,
                ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
                    FindBooksQuery {
                        client,
                        params: [title],
                        stmt: &mut self.0,
                        extractor: |row| super::FindBooksBorrowed {
                            name: row.get(0),
                            author: row.get(1),
                        },
                        mapper: |it| <super::FindBooks>::from(it),
                    }
                }
            }
            pub fn params_use_twice() -> ParamsUseTwiceStmt {
                ParamsUseTwiceStmt(cornucopia_sync::private::Stmt::new(
                    "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
                ))
            }
            pub struct ParamsUseTwiceStmt(cornucopia_sync::private::Stmt);
            impl ParamsUseTwiceStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a T1,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[name])
                }
            }
            pub fn params_order() -> ParamsOrderStmt {
                ParamsOrderStmt(cornucopia_sync::private::Stmt::new(
                    "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
                ))
            }
            pub struct ParamsOrderStmt(cornucopia_sync::private::Stmt);
            impl ParamsOrderStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    c: &'a i32,
                    a: &'a i32,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[c, a])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::ParamsOrderParams,
                    Result<u64, postgres::Error>,
                    C,
                > for ParamsOrderStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::ParamsOrderParams,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.c, &params.a)
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct SelectBookQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::SelectBookBorrowed,
                mapper: fn(super::SelectBookBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> SelectBookQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::SelectBookBorrowed) -> R,
                ) -> SelectBookQuery<'a, C, R, N> {
                    SelectBookQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct FindBooksQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::FindBooksBorrowed,
                mapper: fn(super::FindBooksBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> FindBooksQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::FindBooksBorrowed) -> R,
                ) -> FindBooksQuery<'a, C, R, N> {
                    FindBooksQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn insert_book() -> InsertBookStmt {
                InsertBookStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO book (author, name) VALUES ($1, $2)",
                ))
            }
            pub struct InsertBookStmt(cornucopia_async::private::Stmt);
            impl InsertBookStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                >(
                    &'a mut self,
                    client: &'a C,
                    author: &'a Option<T1>,
                    name: &'a T2,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[author, name]).await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                >
                cornucopia_async::Params<
                    'a,
                    super::InsertBookParams<T1, T2>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for InsertBookStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::InsertBookParams<T1, T2>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.author, &params.name))
                }
            }
            pub fn select_book() -> SelectBookStmt {
                SelectBookStmt(cornucopia_async::private::Stmt::new("SELECT * FROM book"))
            }
            pub struct SelectBookStmt(cornucopia_async::private::Stmt);
            impl SelectBookStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> SelectBookQuery<'a, C, super::SelectBook, 0> {
                    SelectBookQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::SelectBookBorrowed {
                            name: row.get(0),
                            author: row.get(1),
                        },
                        mapper: |it| <super::SelectBook>::from(it),
                    }
                }
            }
            pub fn find_books() -> FindBooksStmt {
                FindBooksStmt(cornucopia_async::private::Stmt::new(
                    "SELECT * FROM book WHERE name = ANY ($1)",
                ))
            }
            pub struct FindBooksStmt(cornucopia_async::private::Stmt);
            impl FindBooksStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::ArraySql<Item = T1>,
                >(
                    &'a mut self,
                    client: &'a C,
                    title: &'a T2,
                ) -> FindBooksQuery<'a, C, super::FindBooks, 1> {
                    FindBooksQuery {
                        client,
                        params: [title],
                        stmt: &mut self.0,
                        extractor: |row| super::FindBooksBorrowed {
                            name: row.get(0),
                            author: row.get(1),
                        },
                        mapper: |it| <super::FindBooks>::from(it),
                    }
                }
            }
            pub fn params_use_twice() -> ParamsUseTwiceStmt {
                ParamsUseTwiceStmt(cornucopia_async::private::Stmt::new(
                    "UPDATE book SET name = $1 WHERE length(name) > 42 AND length($1) < 42",
                ))
            }
            pub struct ParamsUseTwiceStmt(cornucopia_async::private::Stmt);
            impl ParamsUseTwiceStmt {
                pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a T1,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[name]).await
                }
            }
            pub fn params_order() -> ParamsOrderStmt {
                ParamsOrderStmt(cornucopia_async::private::Stmt::new(
                    "UPDATE imaginary SET c=$1, a=$2, z=$2, r=$1",
                ))
            }
            pub struct ParamsOrderStmt(cornucopia_async::private::Stmt);
            impl ParamsOrderStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    c: &'a i32,
                    a: &'a i32,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[c, a]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::ParamsOrderParams,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for ParamsOrderStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::ParamsOrderParams,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.c, &params.a))
                }
            }
        }
    }
    pub mod stress {
        #[derive(Debug)]
        pub struct EverythingParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::BytesSql,
            T4: cornucopia_async::JsonSql,
            T5: cornucopia_async::JsonSql,
        > {
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: T1,
            pub varchar_: T2,
            pub bytea_: T3,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: T4,
            pub jsonb_: T5,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
            pub numeric_: rust_decimal::Decimal,
        }
        #[derive(Debug)]
        pub struct EverythingArrayParams<
            T1: cornucopia_async::ArraySql<Item = bool>,
            T2: cornucopia_async::ArraySql<Item = bool>,
            T3: cornucopia_async::ArraySql<Item = i8>,
            T4: cornucopia_async::ArraySql<Item = i16>,
            T5: cornucopia_async::ArraySql<Item = i16>,
            T6: cornucopia_async::ArraySql<Item = i32>,
            T7: cornucopia_async::ArraySql<Item = i32>,
            T8: cornucopia_async::ArraySql<Item = i64>,
            T9: cornucopia_async::ArraySql<Item = i64>,
            T10: cornucopia_async::ArraySql<Item = f32>,
            T11: cornucopia_async::ArraySql<Item = f32>,
            T12: cornucopia_async::ArraySql<Item = f64>,
            T13: cornucopia_async::ArraySql<Item = f64>,
            T14: cornucopia_async::StringSql,
            T15: cornucopia_async::ArraySql<Item = T14>,
            T16: cornucopia_async::StringSql,
            T17: cornucopia_async::ArraySql<Item = T16>,
            T18: cornucopia_async::BytesSql,
            T19: cornucopia_async::ArraySql<Item = T18>,
            T20: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
            T21: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
            T22: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
            T23: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
            T24: cornucopia_async::ArraySql<Item = time::Date>,
            T25: cornucopia_async::ArraySql<Item = time::Time>,
            T26: cornucopia_async::JsonSql,
            T27: cornucopia_async::ArraySql<Item = T26>,
            T28: cornucopia_async::JsonSql,
            T29: cornucopia_async::ArraySql<Item = T28>,
            T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
            T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
            T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
            T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
        > {
            pub bool_: T1,
            pub boolean_: T2,
            pub char_: T3,
            pub smallint_: T4,
            pub int2_: T5,
            pub int_: T6,
            pub int4_: T7,
            pub bingint_: T8,
            pub int8_: T9,
            pub float4_: T10,
            pub real_: T11,
            pub float8_: T12,
            pub double_precision_: T13,
            pub text_: T15,
            pub varchar_: T17,
            pub bytea_: T19,
            pub timestamp_: T20,
            pub timestamp_without_time_zone_: T21,
            pub timestamptz_: T22,
            pub timestamp_with_time_zone_: T23,
            pub date_: T24,
            pub time_: T25,
            pub json_: T27,
            pub jsonb_: T29,
            pub uuid_: T30,
            pub inet_: T31,
            pub macaddr_: T32,
            pub numeric_: T33,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Everything {
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: String,
            pub varchar_: String,
            pub bytea_: Vec<u8>,
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: serde_json::Value,
            pub jsonb_: serde_json::Value,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
            pub numeric_: rust_decimal::Decimal,
        }
        pub struct EverythingBorrowed<'a> {
            pub bool_: bool,
            pub boolean_: bool,
            pub char_: i8,
            pub smallint_: i16,
            pub int2_: i16,
            pub smallserial_: i16,
            pub serial2_: i16,
            pub int_: i32,
            pub int4_: i32,
            pub serial_: i32,
            pub serial4_: i32,
            pub bingint_: i64,
            pub int8_: i64,
            pub bigserial_: i64,
            pub serial8_: i64,
            pub float4_: f32,
            pub real_: f32,
            pub float8_: f64,
            pub double_precision_: f64,
            pub text_: &'a str,
            pub varchar_: &'a str,
            pub bytea_: &'a [u8],
            pub timestamp_: time::PrimitiveDateTime,
            pub timestamp_without_time_zone_: time::PrimitiveDateTime,
            pub timestamptz_: time::OffsetDateTime,
            pub timestamp_with_time_zone_: time::OffsetDateTime,
            pub date_: time::Date,
            pub time_: time::Time,
            pub json_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub jsonb_: postgres_types::Json<&'a serde_json::value::RawValue>,
            pub uuid_: uuid::Uuid,
            pub inet_: std::net::IpAddr,
            pub macaddr_: eui48::MacAddress,
            pub numeric_: rust_decimal::Decimal,
        }
        impl<'a> From<EverythingBorrowed<'a>> for Everything {
            fn from(
                EverythingBorrowed {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }: EverythingBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_: text_.into(),
                    varchar_: varchar_.into(),
                    bytea_: bytea_.into(),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: serde_json::from_str(json_.0.get()).unwrap(),
                    jsonb_: serde_json::from_str(jsonb_.0.get()).unwrap(),
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingNull {
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub char_: Option<i8>,
            pub smallint_: Option<i16>,
            pub int2_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub serial2_: Option<i16>,
            pub int_: Option<i32>,
            pub int4_: Option<i32>,
            pub serial_: Option<i32>,
            pub serial4_: Option<i32>,
            pub bingint_: Option<i64>,
            pub int8_: Option<i64>,
            pub bigserial_: Option<i64>,
            pub serial8_: Option<i64>,
            pub float4_: Option<f32>,
            pub real_: Option<f32>,
            pub float8_: Option<f64>,
            pub double_precision_: Option<f64>,
            pub text_: Option<String>,
            pub varchar_: Option<String>,
            pub bytea_: Option<Vec<u8>>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub date_: Option<time::Date>,
            pub time_: Option<time::Time>,
            pub json_: Option<serde_json::Value>,
            pub jsonb_: Option<serde_json::Value>,
            pub uuid_: Option<uuid::Uuid>,
            pub inet_: Option<std::net::IpAddr>,
            pub macaddr_: Option<eui48::MacAddress>,
            pub numeric_: Option<rust_decimal::Decimal>,
        }
        pub struct EverythingNullBorrowed<'a> {
            pub bool_: Option<bool>,
            pub boolean_: Option<bool>,
            pub char_: Option<i8>,
            pub smallint_: Option<i16>,
            pub int2_: Option<i16>,
            pub smallserial_: Option<i16>,
            pub serial2_: Option<i16>,
            pub int_: Option<i32>,
            pub int4_: Option<i32>,
            pub serial_: Option<i32>,
            pub serial4_: Option<i32>,
            pub bingint_: Option<i64>,
            pub int8_: Option<i64>,
            pub bigserial_: Option<i64>,
            pub serial8_: Option<i64>,
            pub float4_: Option<f32>,
            pub real_: Option<f32>,
            pub float8_: Option<f64>,
            pub double_precision_: Option<f64>,
            pub text_: Option<&'a str>,
            pub varchar_: Option<&'a str>,
            pub bytea_: Option<&'a [u8]>,
            pub timestamp_: Option<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Option<time::PrimitiveDateTime>,
            pub timestamptz_: Option<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Option<time::OffsetDateTime>,
            pub date_: Option<time::Date>,
            pub time_: Option<time::Time>,
            pub json_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub jsonb_: Option<postgres_types::Json<&'a serde_json::value::RawValue>>,
            pub uuid_: Option<uuid::Uuid>,
            pub inet_: Option<std::net::IpAddr>,
            pub macaddr_: Option<eui48::MacAddress>,
            pub numeric_: Option<rust_decimal::Decimal>,
        }
        impl<'a> From<EverythingNullBorrowed<'a>> for EverythingNull {
            fn from(
                EverythingNullBorrowed {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }: EverythingNullBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    smallserial_,
                    serial2_,
                    int_,
                    int4_,
                    serial_,
                    serial4_,
                    bingint_,
                    int8_,
                    bigserial_,
                    serial8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_: text_.map(|v| v.into()),
                    varchar_: varchar_.map(|v| v.into()),
                    bytea_: bytea_.map(|v| v.into()),
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_: json_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    jsonb_: jsonb_.map(|v| serde_json::from_str(v.0.get()).unwrap()),
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingArray {
            pub bool_: Vec<bool>,
            pub boolean_: Vec<bool>,
            pub char_: Vec<i8>,
            pub smallint_: Vec<i16>,
            pub int2_: Vec<i16>,
            pub int_: Vec<i32>,
            pub int4_: Vec<i32>,
            pub bingint_: Vec<i64>,
            pub int8_: Vec<i64>,
            pub float4_: Vec<f32>,
            pub real_: Vec<f32>,
            pub float8_: Vec<f64>,
            pub double_precision_: Vec<f64>,
            pub text_: Vec<String>,
            pub varchar_: Vec<String>,
            pub bytea_: Vec<Vec<u8>>,
            pub timestamp_: Vec<time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_: Vec<time::PrimitiveDateTime>,
            pub timestamptz_: Vec<time::OffsetDateTime>,
            pub timestamp_with_time_zone_: Vec<time::OffsetDateTime>,
            pub date_: Vec<time::Date>,
            pub time_: Vec<time::Time>,
            pub json_: Vec<serde_json::Value>,
            pub jsonb_: Vec<serde_json::Value>,
            pub uuid_: Vec<uuid::Uuid>,
            pub inet_: Vec<std::net::IpAddr>,
            pub macaddr_: Vec<eui48::MacAddress>,
            pub numeric_: Vec<rust_decimal::Decimal>,
        }
        pub struct EverythingArrayBorrowed<'a> {
            pub bool_: cornucopia_async::ArrayIterator<'a, bool>,
            pub boolean_: cornucopia_async::ArrayIterator<'a, bool>,
            pub char_: cornucopia_async::ArrayIterator<'a, i8>,
            pub smallint_: cornucopia_async::ArrayIterator<'a, i16>,
            pub int2_: cornucopia_async::ArrayIterator<'a, i16>,
            pub int_: cornucopia_async::ArrayIterator<'a, i32>,
            pub int4_: cornucopia_async::ArrayIterator<'a, i32>,
            pub bingint_: cornucopia_async::ArrayIterator<'a, i64>,
            pub int8_: cornucopia_async::ArrayIterator<'a, i64>,
            pub float4_: cornucopia_async::ArrayIterator<'a, f32>,
            pub real_: cornucopia_async::ArrayIterator<'a, f32>,
            pub float8_: cornucopia_async::ArrayIterator<'a, f64>,
            pub double_precision_: cornucopia_async::ArrayIterator<'a, f64>,
            pub text_: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub varchar_: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub bytea_: cornucopia_async::ArrayIterator<'a, &'a [u8]>,
            pub timestamp_: cornucopia_async::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamp_without_time_zone_:
                cornucopia_async::ArrayIterator<'a, time::PrimitiveDateTime>,
            pub timestamptz_: cornucopia_async::ArrayIterator<'a, time::OffsetDateTime>,
            pub timestamp_with_time_zone_:
                cornucopia_async::ArrayIterator<'a, time::OffsetDateTime>,
            pub date_: cornucopia_async::ArrayIterator<'a, time::Date>,
            pub time_: cornucopia_async::ArrayIterator<'a, time::Time>,
            pub json_: cornucopia_async::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub jsonb_: cornucopia_async::ArrayIterator<
                'a,
                postgres_types::Json<&'a serde_json::value::RawValue>,
            >,
            pub uuid_: cornucopia_async::ArrayIterator<'a, uuid::Uuid>,
            pub inet_: cornucopia_async::ArrayIterator<'a, std::net::IpAddr>,
            pub macaddr_: cornucopia_async::ArrayIterator<'a, eui48::MacAddress>,
            pub numeric_: cornucopia_async::ArrayIterator<'a, rust_decimal::Decimal>,
        }
        impl<'a> From<EverythingArrayBorrowed<'a>> for EverythingArray {
            fn from(
                EverythingArrayBorrowed {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    int_,
                    int4_,
                    bingint_,
                    int8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }: EverythingArrayBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_: bool_.map(|v| v).collect(),
                    boolean_: boolean_.map(|v| v).collect(),
                    char_: char_.map(|v| v).collect(),
                    smallint_: smallint_.map(|v| v).collect(),
                    int2_: int2_.map(|v| v).collect(),
                    int_: int_.map(|v| v).collect(),
                    int4_: int4_.map(|v| v).collect(),
                    bingint_: bingint_.map(|v| v).collect(),
                    int8_: int8_.map(|v| v).collect(),
                    float4_: float4_.map(|v| v).collect(),
                    real_: real_.map(|v| v).collect(),
                    float8_: float8_.map(|v| v).collect(),
                    double_precision_: double_precision_.map(|v| v).collect(),
                    text_: text_.map(|v| v.into()).collect(),
                    varchar_: varchar_.map(|v| v.into()).collect(),
                    bytea_: bytea_.map(|v| v.into()).collect(),
                    timestamp_: timestamp_.map(|v| v).collect(),
                    timestamp_without_time_zone_: timestamp_without_time_zone_.map(|v| v).collect(),
                    timestamptz_: timestamptz_.map(|v| v).collect(),
                    timestamp_with_time_zone_: timestamp_with_time_zone_.map(|v| v).collect(),
                    date_: date_.map(|v| v).collect(),
                    time_: time_.map(|v| v).collect(),
                    json_: json_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    jsonb_: jsonb_
                        .map(|v| serde_json::from_str(v.0.get()).unwrap())
                        .collect(),
                    uuid_: uuid_.map(|v| v).collect(),
                    inet_: inet_.map(|v| v).collect(),
                    macaddr_: macaddr_.map(|v| v).collect(),
                    numeric_: numeric_.map(|v| v).collect(),
                }
            }
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct EverythingArrayNull {
            pub bool_: Option<Vec<bool>>,
            pub boolean_: Option<Vec<bool>>,
            pub char_: Option<Vec<i8>>,
            pub smallint_: Option<Vec<i16>>,
            pub int2_: Option<Vec<i16>>,
            pub int_: Option<Vec<i32>>,
            pub int4_: Option<Vec<i32>>,
            pub bingint_: Option<Vec<i64>>,
            pub int8_: Option<Vec<i64>>,
            pub float4_: Option<Vec<f32>>,
            pub real_: Option<Vec<f32>>,
            pub float8_: Option<Vec<f64>>,
            pub double_precision_: Option<Vec<f64>>,
            pub text_: Option<Vec<String>>,
            pub varchar_: Option<Vec<String>>,
            pub bytea_: Option<Vec<Vec<u8>>>,
            pub timestamp_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamp_without_time_zone_: Option<Vec<time::PrimitiveDateTime>>,
            pub timestamptz_: Option<Vec<time::OffsetDateTime>>,
            pub timestamp_with_time_zone_: Option<Vec<time::OffsetDateTime>>,
            pub date_: Option<Vec<time::Date>>,
            pub time_: Option<Vec<time::Time>>,
            pub json_: Option<Vec<serde_json::Value>>,
            pub jsonb_: Option<Vec<serde_json::Value>>,
            pub uuid_: Option<Vec<uuid::Uuid>>,
            pub inet_: Option<Vec<std::net::IpAddr>>,
            pub macaddr_: Option<Vec<eui48::MacAddress>>,
            pub numeric_: Option<Vec<rust_decimal::Decimal>>,
        }
        pub struct EverythingArrayNullBorrowed<'a> {
            pub bool_: Option<cornucopia_async::ArrayIterator<'a, bool>>,
            pub boolean_: Option<cornucopia_async::ArrayIterator<'a, bool>>,
            pub char_: Option<cornucopia_async::ArrayIterator<'a, i8>>,
            pub smallint_: Option<cornucopia_async::ArrayIterator<'a, i16>>,
            pub int2_: Option<cornucopia_async::ArrayIterator<'a, i16>>,
            pub int_: Option<cornucopia_async::ArrayIterator<'a, i32>>,
            pub int4_: Option<cornucopia_async::ArrayIterator<'a, i32>>,
            pub bingint_: Option<cornucopia_async::ArrayIterator<'a, i64>>,
            pub int8_: Option<cornucopia_async::ArrayIterator<'a, i64>>,
            pub float4_: Option<cornucopia_async::ArrayIterator<'a, f32>>,
            pub real_: Option<cornucopia_async::ArrayIterator<'a, f32>>,
            pub float8_: Option<cornucopia_async::ArrayIterator<'a, f64>>,
            pub double_precision_: Option<cornucopia_async::ArrayIterator<'a, f64>>,
            pub text_: Option<cornucopia_async::ArrayIterator<'a, &'a str>>,
            pub varchar_: Option<cornucopia_async::ArrayIterator<'a, &'a str>>,
            pub bytea_: Option<cornucopia_async::ArrayIterator<'a, &'a [u8]>>,
            pub timestamp_: Option<cornucopia_async::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamp_without_time_zone_:
                Option<cornucopia_async::ArrayIterator<'a, time::PrimitiveDateTime>>,
            pub timestamptz_: Option<cornucopia_async::ArrayIterator<'a, time::OffsetDateTime>>,
            pub timestamp_with_time_zone_:
                Option<cornucopia_async::ArrayIterator<'a, time::OffsetDateTime>>,
            pub date_: Option<cornucopia_async::ArrayIterator<'a, time::Date>>,
            pub time_: Option<cornucopia_async::ArrayIterator<'a, time::Time>>,
            pub json_: Option<
                cornucopia_async::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub jsonb_: Option<
                cornucopia_async::ArrayIterator<
                    'a,
                    postgres_types::Json<&'a serde_json::value::RawValue>,
                >,
            >,
            pub uuid_: Option<cornucopia_async::ArrayIterator<'a, uuid::Uuid>>,
            pub inet_: Option<cornucopia_async::ArrayIterator<'a, std::net::IpAddr>>,
            pub macaddr_: Option<cornucopia_async::ArrayIterator<'a, eui48::MacAddress>>,
            pub numeric_: Option<cornucopia_async::ArrayIterator<'a, rust_decimal::Decimal>>,
        }
        impl<'a> From<EverythingArrayNullBorrowed<'a>> for EverythingArrayNull {
            fn from(
                EverythingArrayNullBorrowed {
                    bool_,
                    boolean_,
                    char_,
                    smallint_,
                    int2_,
                    int_,
                    int4_,
                    bingint_,
                    int8_,
                    float4_,
                    real_,
                    float8_,
                    double_precision_,
                    text_,
                    varchar_,
                    bytea_,
                    timestamp_,
                    timestamp_without_time_zone_,
                    timestamptz_,
                    timestamp_with_time_zone_,
                    date_,
                    time_,
                    json_,
                    jsonb_,
                    uuid_,
                    inet_,
                    macaddr_,
                    numeric_,
                }: EverythingArrayNullBorrowed<'a>,
            ) -> Self {
                Self {
                    bool_: bool_.map(|v| v.map(|v| v).collect()),
                    boolean_: boolean_.map(|v| v.map(|v| v).collect()),
                    char_: char_.map(|v| v.map(|v| v).collect()),
                    smallint_: smallint_.map(|v| v.map(|v| v).collect()),
                    int2_: int2_.map(|v| v.map(|v| v).collect()),
                    int_: int_.map(|v| v.map(|v| v).collect()),
                    int4_: int4_.map(|v| v.map(|v| v).collect()),
                    bingint_: bingint_.map(|v| v.map(|v| v).collect()),
                    int8_: int8_.map(|v| v.map(|v| v).collect()),
                    float4_: float4_.map(|v| v.map(|v| v).collect()),
                    real_: real_.map(|v| v.map(|v| v).collect()),
                    float8_: float8_.map(|v| v.map(|v| v).collect()),
                    double_precision_: double_precision_.map(|v| v.map(|v| v).collect()),
                    text_: text_.map(|v| v.map(|v| v.into()).collect()),
                    varchar_: varchar_.map(|v| v.map(|v| v.into()).collect()),
                    bytea_: bytea_.map(|v| v.map(|v| v.into()).collect()),
                    timestamp_: timestamp_.map(|v| v.map(|v| v).collect()),
                    timestamp_without_time_zone_: timestamp_without_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    timestamptz_: timestamptz_.map(|v| v.map(|v| v).collect()),
                    timestamp_with_time_zone_: timestamp_with_time_zone_
                        .map(|v| v.map(|v| v).collect()),
                    date_: date_.map(|v| v.map(|v| v).collect()),
                    time_: time_.map(|v| v.map(|v| v).collect()),
                    json_: json_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    jsonb_: jsonb_.map(|v| {
                        v.map(|v| serde_json::from_str(v.0.get()).unwrap())
                            .collect()
                    }),
                    uuid_: uuid_.map(|v| v.map(|v| v).collect()),
                    inet_: inet_.map(|v| v.map(|v| v).collect()),
                    macaddr_: macaddr_.map(|v| v.map(|v| v).collect()),
                    numeric_: numeric_.map(|v| v.map(|v| v).collect()),
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::EverythingBorrowed,
                mapper: fn(super::EverythingBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingBorrowed) -> R,
                ) -> EverythingQuery<'a, C, R, N> {
                    EverythingQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::EverythingNullBorrowed,
                mapper: fn(super::EverythingNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingNullBorrowed) -> R,
                ) -> EverythingNullQuery<'a, C, R, N> {
                    EverythingNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::EverythingArrayBorrowed,
                mapper: fn(super::EverythingArrayBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingArrayBorrowed) -> R,
                ) -> EverythingArrayQuery<'a, C, R, N> {
                    EverythingArrayQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::EverythingArrayNullBorrowed,
                mapper: fn(super::EverythingArrayNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingArrayNullBorrowed) -> R,
                ) -> EverythingArrayNullQuery<'a, C, R, N> {
                    EverythingArrayNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct PublicNightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(
                    &postgres::Row,
                )
                    -> super::super::super::types::public::NightmareCompositeBorrowed,
                mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicNightmareCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> R,
                ) -> PublicNightmareCompositeQuery<'a, C, R, N> {
                    PublicNightmareCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn select_everything() -> SelectEverythingStmt {
                SelectEverythingStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT
    *
FROM
    Everything",
                ))
            }
            pub struct SelectEverythingStmt(cornucopia_sync::private::Stmt);
            impl SelectEverythingStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> EverythingQuery<'a, C, super::Everything, 0> {
                    EverythingQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            smallserial_: row.get(5),
                            serial2_: row.get(6),
                            int_: row.get(7),
                            int4_: row.get(8),
                            serial_: row.get(9),
                            serial4_: row.get(10),
                            bingint_: row.get(11),
                            int8_: row.get(12),
                            bigserial_: row.get(13),
                            serial8_: row.get(14),
                            float4_: row.get(15),
                            real_: row.get(16),
                            float8_: row.get(17),
                            double_precision_: row.get(18),
                            text_: row.get(19),
                            varchar_: row.get(20),
                            bytea_: row.get(21),
                            timestamp_: row.get(22),
                            timestamp_without_time_zone_: row.get(23),
                            timestamptz_: row.get(24),
                            timestamp_with_time_zone_: row.get(25),
                            date_: row.get(26),
                            time_: row.get(27),
                            json_: row.get(28),
                            jsonb_: row.get(29),
                            uuid_: row.get(30),
                            inet_: row.get(31),
                            macaddr_: row.get(32),
                            numeric_: row.get(33),
                        },
                        mapper: |it| <super::Everything>::from(it),
                    }
                }
            }
            pub fn select_everything_null() -> SelectEverythingNullStmt {
                SelectEverythingNullStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT
    *
FROM
    Everything",
                ))
            }
            pub struct SelectEverythingNullStmt(cornucopia_sync::private::Stmt);
            impl SelectEverythingNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
                    EverythingNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingNullBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            smallserial_: row.get(5),
                            serial2_: row.get(6),
                            int_: row.get(7),
                            int4_: row.get(8),
                            serial_: row.get(9),
                            serial4_: row.get(10),
                            bingint_: row.get(11),
                            int8_: row.get(12),
                            bigserial_: row.get(13),
                            serial8_: row.get(14),
                            float4_: row.get(15),
                            real_: row.get(16),
                            float8_: row.get(17),
                            double_precision_: row.get(18),
                            text_: row.get(19),
                            varchar_: row.get(20),
                            bytea_: row.get(21),
                            timestamp_: row.get(22),
                            timestamp_without_time_zone_: row.get(23),
                            timestamptz_: row.get(24),
                            timestamp_with_time_zone_: row.get(25),
                            date_: row.get(26),
                            time_: row.get(27),
                            json_: row.get(28),
                            jsonb_: row.get(29),
                            uuid_: row.get(30),
                            inet_: row.get(31),
                            macaddr_: row.get(32),
                            numeric_: row.get(33),
                        },
                        mapper: |it| <super::EverythingNull>::from(it),
                    }
                }
            }
            pub fn insert_everything() -> InsertEverythingStmt {
                InsertEverythingStmt(cornucopia_sync::private::Stmt::new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)"))
            }
            pub struct InsertEverythingStmt(cornucopia_sync::private::Stmt);
            impl InsertEverythingStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                    T3: cornucopia_sync::BytesSql,
                    T4: cornucopia_sync::JsonSql,
                    T5: cornucopia_sync::JsonSql,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    bool_: &'a bool,
                    boolean_: &'a bool,
                    char_: &'a i8,
                    smallint_: &'a i16,
                    int2_: &'a i16,
                    smallserial_: &'a i16,
                    serial2_: &'a i16,
                    int_: &'a i32,
                    int4_: &'a i32,
                    serial_: &'a i32,
                    serial4_: &'a i32,
                    bingint_: &'a i64,
                    int8_: &'a i64,
                    bigserial_: &'a i64,
                    serial8_: &'a i64,
                    float4_: &'a f32,
                    real_: &'a f32,
                    float8_: &'a f64,
                    double_precision_: &'a f64,
                    text_: &'a T1,
                    varchar_: &'a T2,
                    bytea_: &'a T3,
                    timestamp_: &'a time::PrimitiveDateTime,
                    timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
                    timestamptz_: &'a time::OffsetDateTime,
                    timestamp_with_time_zone_: &'a time::OffsetDateTime,
                    date_: &'a time::Date,
                    time_: &'a time::Time,
                    json_: &'a T4,
                    jsonb_: &'a T5,
                    uuid_: &'a uuid::Uuid,
                    inet_: &'a std::net::IpAddr,
                    macaddr_: &'a eui48::MacAddress,
                    numeric_: &'a rust_decimal::Decimal,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(
                        stmt,
                        &[
                            bool_,
                            boolean_,
                            char_,
                            smallint_,
                            int2_,
                            smallserial_,
                            serial2_,
                            int_,
                            int4_,
                            serial_,
                            serial4_,
                            bingint_,
                            int8_,
                            bigserial_,
                            serial8_,
                            float4_,
                            real_,
                            float8_,
                            double_precision_,
                            text_,
                            varchar_,
                            bytea_,
                            timestamp_,
                            timestamp_without_time_zone_,
                            timestamptz_,
                            timestamp_with_time_zone_,
                            date_,
                            time_,
                            json_,
                            jsonb_,
                            uuid_,
                            inet_,
                            macaddr_,
                            numeric_,
                        ],
                    )
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::StringSql,
                    T2: cornucopia_sync::StringSql,
                    T3: cornucopia_sync::BytesSql,
                    T4: cornucopia_sync::JsonSql,
                    T5: cornucopia_sync::JsonSql,
                >
                cornucopia_sync::Params<
                    'a,
                    super::EverythingParams<T1, T2, T3, T4, T5>,
                    Result<u64, postgres::Error>,
                    C,
                > for InsertEverythingStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
                ) -> Result<u64, postgres::Error> {
                    self.bind(
                        client,
                        &params.bool_,
                        &params.boolean_,
                        &params.char_,
                        &params.smallint_,
                        &params.int2_,
                        &params.smallserial_,
                        &params.serial2_,
                        &params.int_,
                        &params.int4_,
                        &params.serial_,
                        &params.serial4_,
                        &params.bingint_,
                        &params.int8_,
                        &params.bigserial_,
                        &params.serial8_,
                        &params.float4_,
                        &params.real_,
                        &params.float8_,
                        &params.double_precision_,
                        &params.text_,
                        &params.varchar_,
                        &params.bytea_,
                        &params.timestamp_,
                        &params.timestamp_without_time_zone_,
                        &params.timestamptz_,
                        &params.timestamp_with_time_zone_,
                        &params.date_,
                        &params.time_,
                        &params.json_,
                        &params.jsonb_,
                        &params.uuid_,
                        &params.inet_,
                        &params.macaddr_,
                        &params.numeric_,
                    )
                }
            }
            pub fn select_everything_array() -> SelectEverythingArrayStmt {
                SelectEverythingArrayStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT
    *
FROM
    EverythingArray",
                ))
            }
            pub struct SelectEverythingArrayStmt(cornucopia_sync::private::Stmt);
            impl SelectEverythingArrayStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
                    EverythingArrayQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingArrayBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            int_: row.get(5),
                            int4_: row.get(6),
                            bingint_: row.get(7),
                            int8_: row.get(8),
                            float4_: row.get(9),
                            real_: row.get(10),
                            float8_: row.get(11),
                            double_precision_: row.get(12),
                            text_: row.get(13),
                            varchar_: row.get(14),
                            bytea_: row.get(15),
                            timestamp_: row.get(16),
                            timestamp_without_time_zone_: row.get(17),
                            timestamptz_: row.get(18),
                            timestamp_with_time_zone_: row.get(19),
                            date_: row.get(20),
                            time_: row.get(21),
                            json_: row.get(22),
                            jsonb_: row.get(23),
                            uuid_: row.get(24),
                            inet_: row.get(25),
                            macaddr_: row.get(26),
                            numeric_: row.get(27),
                        },
                        mapper: |it| <super::EverythingArray>::from(it),
                    }
                }
            }
            pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
                SelectEverythingArrayNullStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT
    *
FROM
    EverythingArray",
                ))
            }
            pub struct SelectEverythingArrayNullStmt(cornucopia_sync::private::Stmt);
            impl SelectEverythingArrayNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0>
                {
                    EverythingArrayNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingArrayNullBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            int_: row.get(5),
                            int4_: row.get(6),
                            bingint_: row.get(7),
                            int8_: row.get(8),
                            float4_: row.get(9),
                            real_: row.get(10),
                            float8_: row.get(11),
                            double_precision_: row.get(12),
                            text_: row.get(13),
                            varchar_: row.get(14),
                            bytea_: row.get(15),
                            timestamp_: row.get(16),
                            timestamp_without_time_zone_: row.get(17),
                            timestamptz_: row.get(18),
                            timestamp_with_time_zone_: row.get(19),
                            date_: row.get(20),
                            time_: row.get(21),
                            json_: row.get(22),
                            jsonb_: row.get(23),
                            uuid_: row.get(24),
                            inet_: row.get(25),
                            macaddr_: row.get(26),
                            numeric_: row.get(27),
                        },
                        mapper: |it| <super::EverythingArrayNull>::from(it),
                    }
                }
            }
            pub fn insert_everything_array() -> InsertEverythingArrayStmt {
                InsertEverythingArrayStmt(cornucopia_sync::private::Stmt::new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)"))
            }
            pub struct InsertEverythingArrayStmt(cornucopia_sync::private::Stmt);
            impl InsertEverythingArrayStmt {
                pub fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::ArraySql<Item = bool>,
                    T2: cornucopia_sync::ArraySql<Item = bool>,
                    T3: cornucopia_sync::ArraySql<Item = i8>,
                    T4: cornucopia_sync::ArraySql<Item = i16>,
                    T5: cornucopia_sync::ArraySql<Item = i16>,
                    T6: cornucopia_sync::ArraySql<Item = i32>,
                    T7: cornucopia_sync::ArraySql<Item = i32>,
                    T8: cornucopia_sync::ArraySql<Item = i64>,
                    T9: cornucopia_sync::ArraySql<Item = i64>,
                    T10: cornucopia_sync::ArraySql<Item = f32>,
                    T11: cornucopia_sync::ArraySql<Item = f32>,
                    T12: cornucopia_sync::ArraySql<Item = f64>,
                    T13: cornucopia_sync::ArraySql<Item = f64>,
                    T14: cornucopia_sync::StringSql,
                    T15: cornucopia_sync::ArraySql<Item = T14>,
                    T16: cornucopia_sync::StringSql,
                    T17: cornucopia_sync::ArraySql<Item = T16>,
                    T18: cornucopia_sync::BytesSql,
                    T19: cornucopia_sync::ArraySql<Item = T18>,
                    T20: cornucopia_sync::ArraySql<Item = time::PrimitiveDateTime>,
                    T21: cornucopia_sync::ArraySql<Item = time::PrimitiveDateTime>,
                    T22: cornucopia_sync::ArraySql<Item = time::OffsetDateTime>,
                    T23: cornucopia_sync::ArraySql<Item = time::OffsetDateTime>,
                    T24: cornucopia_sync::ArraySql<Item = time::Date>,
                    T25: cornucopia_sync::ArraySql<Item = time::Time>,
                    T26: cornucopia_sync::JsonSql,
                    T27: cornucopia_sync::ArraySql<Item = T26>,
                    T28: cornucopia_sync::JsonSql,
                    T29: cornucopia_sync::ArraySql<Item = T28>,
                    T30: cornucopia_sync::ArraySql<Item = uuid::Uuid>,
                    T31: cornucopia_sync::ArraySql<Item = std::net::IpAddr>,
                    T32: cornucopia_sync::ArraySql<Item = eui48::MacAddress>,
                    T33: cornucopia_sync::ArraySql<Item = rust_decimal::Decimal>,
                >(
                    &'a mut self,
                    client: &'a mut C,
                    bool_: &'a T1,
                    boolean_: &'a T2,
                    char_: &'a T3,
                    smallint_: &'a T4,
                    int2_: &'a T5,
                    int_: &'a T6,
                    int4_: &'a T7,
                    bingint_: &'a T8,
                    int8_: &'a T9,
                    float4_: &'a T10,
                    real_: &'a T11,
                    float8_: &'a T12,
                    double_precision_: &'a T13,
                    text_: &'a T15,
                    varchar_: &'a T17,
                    bytea_: &'a T19,
                    timestamp_: &'a T20,
                    timestamp_without_time_zone_: &'a T21,
                    timestamptz_: &'a T22,
                    timestamp_with_time_zone_: &'a T23,
                    date_: &'a T24,
                    time_: &'a T25,
                    json_: &'a T27,
                    jsonb_: &'a T29,
                    uuid_: &'a T30,
                    inet_: &'a T31,
                    macaddr_: &'a T32,
                    numeric_: &'a T33,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(
                        stmt,
                        &[
                            bool_,
                            boolean_,
                            char_,
                            smallint_,
                            int2_,
                            int_,
                            int4_,
                            bingint_,
                            int8_,
                            float4_,
                            real_,
                            float8_,
                            double_precision_,
                            text_,
                            varchar_,
                            bytea_,
                            timestamp_,
                            timestamp_without_time_zone_,
                            timestamptz_,
                            timestamp_with_time_zone_,
                            date_,
                            time_,
                            json_,
                            jsonb_,
                            uuid_,
                            inet_,
                            macaddr_,
                            numeric_,
                        ],
                    )
                }
            }
            impl<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_sync::ArraySql<Item = bool>,
                    T2: cornucopia_sync::ArraySql<Item = bool>,
                    T3: cornucopia_sync::ArraySql<Item = i8>,
                    T4: cornucopia_sync::ArraySql<Item = i16>,
                    T5: cornucopia_sync::ArraySql<Item = i16>,
                    T6: cornucopia_sync::ArraySql<Item = i32>,
                    T7: cornucopia_sync::ArraySql<Item = i32>,
                    T8: cornucopia_sync::ArraySql<Item = i64>,
                    T9: cornucopia_sync::ArraySql<Item = i64>,
                    T10: cornucopia_sync::ArraySql<Item = f32>,
                    T11: cornucopia_sync::ArraySql<Item = f32>,
                    T12: cornucopia_sync::ArraySql<Item = f64>,
                    T13: cornucopia_sync::ArraySql<Item = f64>,
                    T14: cornucopia_sync::StringSql,
                    T15: cornucopia_sync::ArraySql<Item = T14>,
                    T16: cornucopia_sync::StringSql,
                    T17: cornucopia_sync::ArraySql<Item = T16>,
                    T18: cornucopia_sync::BytesSql,
                    T19: cornucopia_sync::ArraySql<Item = T18>,
                    T20: cornucopia_sync::ArraySql<Item = time::PrimitiveDateTime>,
                    T21: cornucopia_sync::ArraySql<Item = time::PrimitiveDateTime>,
                    T22: cornucopia_sync::ArraySql<Item = time::OffsetDateTime>,
                    T23: cornucopia_sync::ArraySql<Item = time::OffsetDateTime>,
                    T24: cornucopia_sync::ArraySql<Item = time::Date>,
                    T25: cornucopia_sync::ArraySql<Item = time::Time>,
                    T26: cornucopia_sync::JsonSql,
                    T27: cornucopia_sync::ArraySql<Item = T26>,
                    T28: cornucopia_sync::JsonSql,
                    T29: cornucopia_sync::ArraySql<Item = T28>,
                    T30: cornucopia_sync::ArraySql<Item = uuid::Uuid>,
                    T31: cornucopia_sync::ArraySql<Item = std::net::IpAddr>,
                    T32: cornucopia_sync::ArraySql<Item = eui48::MacAddress>,
                    T33: cornucopia_sync::ArraySql<Item = rust_decimal::Decimal>,
                >
                cornucopia_sync::Params<
                    'a,
                    super::EverythingArrayParams<
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                        T21,
                        T22,
                        T23,
                        T24,
                        T25,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T33,
                    >,
                    Result<u64, postgres::Error>,
                    C,
                > for InsertEverythingArrayStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::EverythingArrayParams<
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                        T21,
                        T22,
                        T23,
                        T24,
                        T25,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T33,
                    >,
                ) -> Result<u64, postgres::Error> {
                    self.bind(
                        client,
                        &params.bool_,
                        &params.boolean_,
                        &params.char_,
                        &params.smallint_,
                        &params.int2_,
                        &params.int_,
                        &params.int4_,
                        &params.bingint_,
                        &params.int8_,
                        &params.float4_,
                        &params.real_,
                        &params.float8_,
                        &params.double_precision_,
                        &params.text_,
                        &params.varchar_,
                        &params.bytea_,
                        &params.timestamp_,
                        &params.timestamp_without_time_zone_,
                        &params.timestamptz_,
                        &params.timestamp_with_time_zone_,
                        &params.date_,
                        &params.time_,
                        &params.json_,
                        &params.jsonb_,
                        &params.uuid_,
                        &params.inet_,
                        &params.macaddr_,
                        &params.numeric_,
                    )
                }
            }
            pub fn select_nightmare() -> SelectNightmareStmt {
                SelectNightmareStmt(cornucopia_sync::private::Stmt::new(
                    "SELECT
    *
FROM
    nightmare",
                ))
            }
            pub struct SelectNightmareStmt(cornucopia_sync::private::Stmt);
            impl SelectNightmareStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> PublicNightmareCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::NightmareComposite,
                    0,
                > {
                    PublicNightmareCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn insert_nightmare() -> InsertNightmareStmt {
                InsertNightmareStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO nightmare (composite)
    VALUES ($1)",
                ))
            }
            pub struct InsertNightmareStmt(cornucopia_sync::private::Stmt);
            impl InsertNightmareStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    composite: &'a super::super::super::types::public::NightmareCompositeParams<'a>,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[composite])
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct EverythingQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::EverythingBorrowed,
                mapper: fn(super::EverythingBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingBorrowed) -> R,
                ) -> EverythingQuery<'a, C, R, N> {
                    EverythingQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct EverythingNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::EverythingNullBorrowed,
                mapper: fn(super::EverythingNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingNullBorrowed) -> R,
                ) -> EverythingNullQuery<'a, C, R, N> {
                    EverythingNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct EverythingArrayQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayBorrowed,
                mapper: fn(super::EverythingArrayBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingArrayQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingArrayBorrowed) -> R,
                ) -> EverythingArrayQuery<'a, C, R, N> {
                    EverythingArrayQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct EverythingArrayNullQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::EverythingArrayNullBorrowed,
                mapper: fn(super::EverythingArrayNullBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> EverythingArrayNullQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::EverythingArrayNullBorrowed) -> R,
                ) -> EverythingArrayNullQuery<'a, C, R, N> {
                    EverythingArrayNullQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct PublicNightmareCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(
                    &tokio_postgres::Row,
                )
                    -> super::super::super::types::public::NightmareCompositeBorrowed,
                mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicNightmareCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::NightmareCompositeBorrowed) -> R,
                ) -> PublicNightmareCompositeQuery<'a, C, R, N> {
                    PublicNightmareCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn select_everything() -> SelectEverythingStmt {
                SelectEverythingStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    *
FROM
    Everything",
                ))
            }
            pub struct SelectEverythingStmt(cornucopia_async::private::Stmt);
            impl SelectEverythingStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> EverythingQuery<'a, C, super::Everything, 0> {
                    EverythingQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            smallserial_: row.get(5),
                            serial2_: row.get(6),
                            int_: row.get(7),
                            int4_: row.get(8),
                            serial_: row.get(9),
                            serial4_: row.get(10),
                            bingint_: row.get(11),
                            int8_: row.get(12),
                            bigserial_: row.get(13),
                            serial8_: row.get(14),
                            float4_: row.get(15),
                            real_: row.get(16),
                            float8_: row.get(17),
                            double_precision_: row.get(18),
                            text_: row.get(19),
                            varchar_: row.get(20),
                            bytea_: row.get(21),
                            timestamp_: row.get(22),
                            timestamp_without_time_zone_: row.get(23),
                            timestamptz_: row.get(24),
                            timestamp_with_time_zone_: row.get(25),
                            date_: row.get(26),
                            time_: row.get(27),
                            json_: row.get(28),
                            jsonb_: row.get(29),
                            uuid_: row.get(30),
                            inet_: row.get(31),
                            macaddr_: row.get(32),
                            numeric_: row.get(33),
                        },
                        mapper: |it| <super::Everything>::from(it),
                    }
                }
            }
            pub fn select_everything_null() -> SelectEverythingNullStmt {
                SelectEverythingNullStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    *
FROM
    Everything",
                ))
            }
            pub struct SelectEverythingNullStmt(cornucopia_async::private::Stmt);
            impl SelectEverythingNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> EverythingNullQuery<'a, C, super::EverythingNull, 0> {
                    EverythingNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingNullBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            smallserial_: row.get(5),
                            serial2_: row.get(6),
                            int_: row.get(7),
                            int4_: row.get(8),
                            serial_: row.get(9),
                            serial4_: row.get(10),
                            bingint_: row.get(11),
                            int8_: row.get(12),
                            bigserial_: row.get(13),
                            serial8_: row.get(14),
                            float4_: row.get(15),
                            real_: row.get(16),
                            float8_: row.get(17),
                            double_precision_: row.get(18),
                            text_: row.get(19),
                            varchar_: row.get(20),
                            bytea_: row.get(21),
                            timestamp_: row.get(22),
                            timestamp_without_time_zone_: row.get(23),
                            timestamptz_: row.get(24),
                            timestamp_with_time_zone_: row.get(25),
                            date_: row.get(26),
                            time_: row.get(27),
                            json_: row.get(28),
                            jsonb_: row.get(29),
                            uuid_: row.get(30),
                            inet_: row.get(31),
                            macaddr_: row.get(32),
                            numeric_: row.get(33),
                        },
                        mapper: |it| <super::EverythingNull>::from(it),
                    }
                }
            }
            pub fn insert_everything() -> InsertEverythingStmt {
                InsertEverythingStmt(cornucopia_async::private::Stmt::new("INSERT INTO Everything (bool_, boolean_, char_, smallint_, int2_, smallserial_, serial2_, int_, int4_, serial_, serial4_, bingint_, int8_, bigserial_, serial8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)"))
            }
            pub struct InsertEverythingStmt(cornucopia_async::private::Stmt);
            impl InsertEverythingStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                    T3: cornucopia_async::BytesSql,
                    T4: cornucopia_async::JsonSql,
                    T5: cornucopia_async::JsonSql,
                >(
                    &'a mut self,
                    client: &'a C,
                    bool_: &'a bool,
                    boolean_: &'a bool,
                    char_: &'a i8,
                    smallint_: &'a i16,
                    int2_: &'a i16,
                    smallserial_: &'a i16,
                    serial2_: &'a i16,
                    int_: &'a i32,
                    int4_: &'a i32,
                    serial_: &'a i32,
                    serial4_: &'a i32,
                    bingint_: &'a i64,
                    int8_: &'a i64,
                    bigserial_: &'a i64,
                    serial8_: &'a i64,
                    float4_: &'a f32,
                    real_: &'a f32,
                    float8_: &'a f64,
                    double_precision_: &'a f64,
                    text_: &'a T1,
                    varchar_: &'a T2,
                    bytea_: &'a T3,
                    timestamp_: &'a time::PrimitiveDateTime,
                    timestamp_without_time_zone_: &'a time::PrimitiveDateTime,
                    timestamptz_: &'a time::OffsetDateTime,
                    timestamp_with_time_zone_: &'a time::OffsetDateTime,
                    date_: &'a time::Date,
                    time_: &'a time::Time,
                    json_: &'a T4,
                    jsonb_: &'a T5,
                    uuid_: &'a uuid::Uuid,
                    inet_: &'a std::net::IpAddr,
                    macaddr_: &'a eui48::MacAddress,
                    numeric_: &'a rust_decimal::Decimal,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client
                        .execute(
                            stmt,
                            &[
                                bool_,
                                boolean_,
                                char_,
                                smallint_,
                                int2_,
                                smallserial_,
                                serial2_,
                                int_,
                                int4_,
                                serial_,
                                serial4_,
                                bingint_,
                                int8_,
                                bigserial_,
                                serial8_,
                                float4_,
                                real_,
                                float8_,
                                double_precision_,
                                text_,
                                varchar_,
                                bytea_,
                                timestamp_,
                                timestamp_without_time_zone_,
                                timestamptz_,
                                timestamp_with_time_zone_,
                                date_,
                                time_,
                                json_,
                                jsonb_,
                                uuid_,
                                inet_,
                                macaddr_,
                                numeric_,
                            ],
                        )
                        .await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::StringSql,
                    T2: cornucopia_async::StringSql,
                    T3: cornucopia_async::BytesSql,
                    T4: cornucopia_async::JsonSql,
                    T5: cornucopia_async::JsonSql,
                >
                cornucopia_async::Params<
                    'a,
                    super::EverythingParams<T1, T2, T3, T4, T5>,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for InsertEverythingStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::EverythingParams<T1, T2, T3, T4, T5>,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(
                        client,
                        &params.bool_,
                        &params.boolean_,
                        &params.char_,
                        &params.smallint_,
                        &params.int2_,
                        &params.smallserial_,
                        &params.serial2_,
                        &params.int_,
                        &params.int4_,
                        &params.serial_,
                        &params.serial4_,
                        &params.bingint_,
                        &params.int8_,
                        &params.bigserial_,
                        &params.serial8_,
                        &params.float4_,
                        &params.real_,
                        &params.float8_,
                        &params.double_precision_,
                        &params.text_,
                        &params.varchar_,
                        &params.bytea_,
                        &params.timestamp_,
                        &params.timestamp_without_time_zone_,
                        &params.timestamptz_,
                        &params.timestamp_with_time_zone_,
                        &params.date_,
                        &params.time_,
                        &params.json_,
                        &params.jsonb_,
                        &params.uuid_,
                        &params.inet_,
                        &params.macaddr_,
                        &params.numeric_,
                    ))
                }
            }
            pub fn select_everything_array() -> SelectEverythingArrayStmt {
                SelectEverythingArrayStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    *
FROM
    EverythingArray",
                ))
            }
            pub struct SelectEverythingArrayStmt(cornucopia_async::private::Stmt);
            impl SelectEverythingArrayStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> EverythingArrayQuery<'a, C, super::EverythingArray, 0> {
                    EverythingArrayQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingArrayBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            int_: row.get(5),
                            int4_: row.get(6),
                            bingint_: row.get(7),
                            int8_: row.get(8),
                            float4_: row.get(9),
                            real_: row.get(10),
                            float8_: row.get(11),
                            double_precision_: row.get(12),
                            text_: row.get(13),
                            varchar_: row.get(14),
                            bytea_: row.get(15),
                            timestamp_: row.get(16),
                            timestamp_without_time_zone_: row.get(17),
                            timestamptz_: row.get(18),
                            timestamp_with_time_zone_: row.get(19),
                            date_: row.get(20),
                            time_: row.get(21),
                            json_: row.get(22),
                            jsonb_: row.get(23),
                            uuid_: row.get(24),
                            inet_: row.get(25),
                            macaddr_: row.get(26),
                            numeric_: row.get(27),
                        },
                        mapper: |it| <super::EverythingArray>::from(it),
                    }
                }
            }
            pub fn select_everything_array_null() -> SelectEverythingArrayNullStmt {
                SelectEverythingArrayNullStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    *
FROM
    EverythingArray",
                ))
            }
            pub struct SelectEverythingArrayNullStmt(cornucopia_async::private::Stmt);
            impl SelectEverythingArrayNullStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> EverythingArrayNullQuery<'a, C, super::EverythingArrayNull, 0>
                {
                    EverythingArrayNullQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::EverythingArrayNullBorrowed {
                            bool_: row.get(0),
                            boolean_: row.get(1),
                            char_: row.get(2),
                            smallint_: row.get(3),
                            int2_: row.get(4),
                            int_: row.get(5),
                            int4_: row.get(6),
                            bingint_: row.get(7),
                            int8_: row.get(8),
                            float4_: row.get(9),
                            real_: row.get(10),
                            float8_: row.get(11),
                            double_precision_: row.get(12),
                            text_: row.get(13),
                            varchar_: row.get(14),
                            bytea_: row.get(15),
                            timestamp_: row.get(16),
                            timestamp_without_time_zone_: row.get(17),
                            timestamptz_: row.get(18),
                            timestamp_with_time_zone_: row.get(19),
                            date_: row.get(20),
                            time_: row.get(21),
                            json_: row.get(22),
                            jsonb_: row.get(23),
                            uuid_: row.get(24),
                            inet_: row.get(25),
                            macaddr_: row.get(26),
                            numeric_: row.get(27),
                        },
                        mapper: |it| <super::EverythingArrayNull>::from(it),
                    }
                }
            }
            pub fn insert_everything_array() -> InsertEverythingArrayStmt {
                InsertEverythingArrayStmt(cornucopia_async::private::Stmt::new("INSERT INTO EverythingArray (bool_, boolean_, char_, smallint_, int2_, int_, int4_, bingint_, int8_, float4_, real_, float8_, double_precision_, text_, varchar_, bytea_, timestamp_, timestamp_without_time_zone_, timestamptz_, timestamp_with_time_zone_, date_, time_, json_, jsonb_, uuid_, inet_, macaddr_, numeric_)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)"))
            }
            pub struct InsertEverythingArrayStmt(cornucopia_async::private::Stmt);
            impl InsertEverythingArrayStmt {
                pub async fn bind<
                    'a,
                    C: GenericClient,
                    T1: cornucopia_async::ArraySql<Item = bool>,
                    T2: cornucopia_async::ArraySql<Item = bool>,
                    T3: cornucopia_async::ArraySql<Item = i8>,
                    T4: cornucopia_async::ArraySql<Item = i16>,
                    T5: cornucopia_async::ArraySql<Item = i16>,
                    T6: cornucopia_async::ArraySql<Item = i32>,
                    T7: cornucopia_async::ArraySql<Item = i32>,
                    T8: cornucopia_async::ArraySql<Item = i64>,
                    T9: cornucopia_async::ArraySql<Item = i64>,
                    T10: cornucopia_async::ArraySql<Item = f32>,
                    T11: cornucopia_async::ArraySql<Item = f32>,
                    T12: cornucopia_async::ArraySql<Item = f64>,
                    T13: cornucopia_async::ArraySql<Item = f64>,
                    T14: cornucopia_async::StringSql,
                    T15: cornucopia_async::ArraySql<Item = T14>,
                    T16: cornucopia_async::StringSql,
                    T17: cornucopia_async::ArraySql<Item = T16>,
                    T18: cornucopia_async::BytesSql,
                    T19: cornucopia_async::ArraySql<Item = T18>,
                    T20: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
                    T21: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
                    T22: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
                    T23: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
                    T24: cornucopia_async::ArraySql<Item = time::Date>,
                    T25: cornucopia_async::ArraySql<Item = time::Time>,
                    T26: cornucopia_async::JsonSql,
                    T27: cornucopia_async::ArraySql<Item = T26>,
                    T28: cornucopia_async::JsonSql,
                    T29: cornucopia_async::ArraySql<Item = T28>,
                    T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
                    T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
                    T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
                    T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
                >(
                    &'a mut self,
                    client: &'a C,
                    bool_: &'a T1,
                    boolean_: &'a T2,
                    char_: &'a T3,
                    smallint_: &'a T4,
                    int2_: &'a T5,
                    int_: &'a T6,
                    int4_: &'a T7,
                    bingint_: &'a T8,
                    int8_: &'a T9,
                    float4_: &'a T10,
                    real_: &'a T11,
                    float8_: &'a T12,
                    double_precision_: &'a T13,
                    text_: &'a T15,
                    varchar_: &'a T17,
                    bytea_: &'a T19,
                    timestamp_: &'a T20,
                    timestamp_without_time_zone_: &'a T21,
                    timestamptz_: &'a T22,
                    timestamp_with_time_zone_: &'a T23,
                    date_: &'a T24,
                    time_: &'a T25,
                    json_: &'a T27,
                    jsonb_: &'a T29,
                    uuid_: &'a T30,
                    inet_: &'a T31,
                    macaddr_: &'a T32,
                    numeric_: &'a T33,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client
                        .execute(
                            stmt,
                            &[
                                bool_,
                                boolean_,
                                char_,
                                smallint_,
                                int2_,
                                int_,
                                int4_,
                                bingint_,
                                int8_,
                                float4_,
                                real_,
                                float8_,
                                double_precision_,
                                text_,
                                varchar_,
                                bytea_,
                                timestamp_,
                                timestamp_without_time_zone_,
                                timestamptz_,
                                timestamp_with_time_zone_,
                                date_,
                                time_,
                                json_,
                                jsonb_,
                                uuid_,
                                inet_,
                                macaddr_,
                                numeric_,
                            ],
                        )
                        .await
                }
            }
            impl<
                    'a,
                    C: GenericClient + Send + Sync,
                    T1: cornucopia_async::ArraySql<Item = bool>,
                    T2: cornucopia_async::ArraySql<Item = bool>,
                    T3: cornucopia_async::ArraySql<Item = i8>,
                    T4: cornucopia_async::ArraySql<Item = i16>,
                    T5: cornucopia_async::ArraySql<Item = i16>,
                    T6: cornucopia_async::ArraySql<Item = i32>,
                    T7: cornucopia_async::ArraySql<Item = i32>,
                    T8: cornucopia_async::ArraySql<Item = i64>,
                    T9: cornucopia_async::ArraySql<Item = i64>,
                    T10: cornucopia_async::ArraySql<Item = f32>,
                    T11: cornucopia_async::ArraySql<Item = f32>,
                    T12: cornucopia_async::ArraySql<Item = f64>,
                    T13: cornucopia_async::ArraySql<Item = f64>,
                    T14: cornucopia_async::StringSql,
                    T15: cornucopia_async::ArraySql<Item = T14>,
                    T16: cornucopia_async::StringSql,
                    T17: cornucopia_async::ArraySql<Item = T16>,
                    T18: cornucopia_async::BytesSql,
                    T19: cornucopia_async::ArraySql<Item = T18>,
                    T20: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
                    T21: cornucopia_async::ArraySql<Item = time::PrimitiveDateTime>,
                    T22: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
                    T23: cornucopia_async::ArraySql<Item = time::OffsetDateTime>,
                    T24: cornucopia_async::ArraySql<Item = time::Date>,
                    T25: cornucopia_async::ArraySql<Item = time::Time>,
                    T26: cornucopia_async::JsonSql,
                    T27: cornucopia_async::ArraySql<Item = T26>,
                    T28: cornucopia_async::JsonSql,
                    T29: cornucopia_async::ArraySql<Item = T28>,
                    T30: cornucopia_async::ArraySql<Item = uuid::Uuid>,
                    T31: cornucopia_async::ArraySql<Item = std::net::IpAddr>,
                    T32: cornucopia_async::ArraySql<Item = eui48::MacAddress>,
                    T33: cornucopia_async::ArraySql<Item = rust_decimal::Decimal>,
                >
                cornucopia_async::Params<
                    'a,
                    super::EverythingArrayParams<
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                        T21,
                        T22,
                        T23,
                        T24,
                        T25,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T33,
                    >,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for InsertEverythingArrayStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::EverythingArrayParams<
                        T1,
                        T2,
                        T3,
                        T4,
                        T5,
                        T6,
                        T7,
                        T8,
                        T9,
                        T10,
                        T11,
                        T12,
                        T13,
                        T14,
                        T15,
                        T16,
                        T17,
                        T18,
                        T19,
                        T20,
                        T21,
                        T22,
                        T23,
                        T24,
                        T25,
                        T26,
                        T27,
                        T28,
                        T29,
                        T30,
                        T31,
                        T32,
                        T33,
                    >,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(
                        client,
                        &params.bool_,
                        &params.boolean_,
                        &params.char_,
                        &params.smallint_,
                        &params.int2_,
                        &params.int_,
                        &params.int4_,
                        &params.bingint_,
                        &params.int8_,
                        &params.float4_,
                        &params.real_,
                        &params.float8_,
                        &params.double_precision_,
                        &params.text_,
                        &params.varchar_,
                        &params.bytea_,
                        &params.timestamp_,
                        &params.timestamp_without_time_zone_,
                        &params.timestamptz_,
                        &params.timestamp_with_time_zone_,
                        &params.date_,
                        &params.time_,
                        &params.json_,
                        &params.jsonb_,
                        &params.uuid_,
                        &params.inet_,
                        &params.macaddr_,
                        &params.numeric_,
                    ))
                }
            }
            pub fn select_nightmare() -> SelectNightmareStmt {
                SelectNightmareStmt(cornucopia_async::private::Stmt::new(
                    "SELECT
    *
FROM
    nightmare",
                ))
            }
            pub struct SelectNightmareStmt(cornucopia_async::private::Stmt);
            impl SelectNightmareStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> PublicNightmareCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::NightmareComposite,
                    0,
                > {
                    PublicNightmareCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn insert_nightmare() -> InsertNightmareStmt {
                InsertNightmareStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO nightmare (composite)
    VALUES ($1)",
                ))
            }
            pub struct InsertNightmareStmt(cornucopia_async::private::Stmt);
            impl InsertNightmareStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    composite: &'a super::super::super::types::public::NightmareCompositeParams<'a>,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[composite]).await
                }
            }
        }
    }
    pub mod syntax {
        #[derive(Debug)]
        pub struct ImplicitCompactParams<T1: cornucopia_async::StringSql> {
            pub name: Option<T1>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct ImplicitSpacedParams<T1: cornucopia_async::StringSql> {
            pub name: Option<T1>,
            pub price: Option<f64>,
        }
        #[derive(Debug)]
        pub struct Params<T1: cornucopia_async::StringSql> {
            pub name: T1,
            pub price: f64,
        }
        #[derive(Debug)]
        pub struct ParamsSpace<T1: cornucopia_async::StringSql> {
            pub name: T1,
            pub price: f64,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySqlParams {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql1Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql2Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql3Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql4Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql6Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql7Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql8Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql9Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct TrickySql10Params {
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct Row {
            pub id: i32,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq, Copy)]
        pub struct RowSpace {
            pub id: i32,
        }
        #[derive(serde::Serialize, Debug, Clone, PartialEq)]
        pub struct Typeof {
            pub trick_y: String,
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        pub struct TypeofBorrowed<'a> {
            pub trick_y: &'a str,
            pub r#async: super::super::types::public::SyntaxComposite,
            pub r#enum: super::super::types::public::SyntaxEnum,
        }
        impl<'a> From<TypeofBorrowed<'a>> for Typeof {
            fn from(
                TypeofBorrowed {
                    trick_y,
                    r#async,
                    r#enum,
                }: TypeofBorrowed<'a>,
            ) -> Self {
                Self {
                    trick_y: trick_y.into(),
                    r#async,
                    r#enum,
                }
            }
        }
        pub mod sync {
            use postgres::{fallible_iterator::FallibleIterator, GenericClient};
            pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(
                    &postgres::Row,
                )
                    -> super::super::super::types::public::CloneCompositeBorrowed,
                mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
                ) -> PublicCloneCompositeQuery<'a, C, R, N> {
                    PublicCloneCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> Option<i32>,
                mapper: fn(Option<i32>) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
                    Optioni32Query {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::Row,
                mapper: fn(super::Row) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
                    RowQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::RowSpace,
                mapper: fn(super::RowSpace) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::RowSpace) -> R,
                ) -> RowSpaceQuery<'a, C, R, N> {
                    RowSpaceQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a mut C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_sync::private::Stmt,
                extractor: fn(&postgres::Row) -> super::TypeofBorrowed,
                mapper: fn(super::TypeofBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::TypeofBorrowed) -> R,
                ) -> TypeofQuery<'a, C, R, N> {
                    TypeofQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub fn one(self) -> Result<T, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    let row = self.client.query_one(stmt, &self.params)?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub fn all(self) -> Result<Vec<T>, postgres::Error> {
                    self.iter()?.collect()
                }
                pub fn opt(self) -> Result<Option<T>, postgres::Error> {
                    let stmt = self.stmt.prepare(self.client)?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub fn iter(
                    self,
                ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error>
                {
                    let stmt = self.stmt.prepare(self.client)?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_sync::private::slice_iter(&self.params))?
                        .iterator()
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))));
                    Ok(it)
                }
            }
            pub fn select_compact() -> SelectCompactStmt {
                SelectCompactStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM clone"))
            }
            pub struct SelectCompactStmt(cornucopia_sync::private::Stmt);
            impl SelectCompactStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn select_spaced() -> SelectSpacedStmt {
                SelectSpacedStmt(cornucopia_sync::private::Stmt::new(
                    "      SELECT * FROM clone ",
                ))
            }
            pub struct SelectSpacedStmt(cornucopia_sync::private::Stmt);
            impl SelectSpacedStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn implicit_compact() -> ImplicitCompactStmt {
                ImplicitCompactStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct ImplicitCompactStmt(cornucopia_sync::private::Stmt);
            impl ImplicitCompactStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a Option<T1>,
                    price: &'a Option<f64>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    Optioni32Query {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<
                    'a,
                    super::ImplicitCompactParams<T1>,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                > for ImplicitCompactStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::ImplicitCompactParams<T1>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn implicit_spaced() -> ImplicitSpacedStmt {
                ImplicitSpacedStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct ImplicitSpacedStmt(cornucopia_sync::private::Stmt);
            impl ImplicitSpacedStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a Option<T1>,
                    price: &'a Option<f64>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    Optioni32Query {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<
                    'a,
                    super::ImplicitSpacedParams<T1>,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                > for ImplicitSpacedStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::ImplicitSpacedParams<T1>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn named_compact() -> NamedCompactStmt {
                NamedCompactStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NamedCompactStmt(cornucopia_sync::private::Stmt);
            impl NamedCompactStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a T1,
                    price: &'a f64,
                ) -> RowQuery<'a, C, super::Row, 2> {
                    RowQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::Row { id: row.get(0) },
                        mapper: |it| <super::Row>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
                for NamedCompactStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::Params<T1>,
                ) -> RowQuery<'a, C, super::Row, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn named_spaced() -> NamedSpacedStmt {
                NamedSpacedStmt(cornucopia_sync::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NamedSpacedStmt(cornucopia_sync::private::Stmt);
            impl NamedSpacedStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_sync::StringSql>(
                    &'a mut self,
                    client: &'a mut C,
                    name: &'a T1,
                    price: &'a f64,
                ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
                    RowSpaceQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::RowSpace { id: row.get(0) },
                        mapper: |it| <super::RowSpace>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_sync::StringSql>
                cornucopia_sync::Params<
                    'a,
                    super::ParamsSpace<T1>,
                    RowSpaceQuery<'a, C, super::RowSpace, 2>,
                    C,
                > for NamedSpacedStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::ParamsSpace<T1>,
                ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn tricky_sql() -> TrickySqlStmt {
                TrickySqlStmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)"))
            }
            pub struct TrickySqlStmt(cornucopia_sync::private::Stmt);
            impl TrickySqlStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<'a, super::TrickySqlParams, Result<u64, postgres::Error>, C>
                for TrickySqlStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySqlParams,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql1() -> TrickySql1Stmt {
                TrickySql1Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)"))
            }
            pub struct TrickySql1Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql1Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql1Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql1Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql1Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql2() -> TrickySql2Stmt {
                TrickySql2Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)"))
            }
            pub struct TrickySql2Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql2Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql2Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql2Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql2Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql3() -> TrickySql3Stmt {
                TrickySql3Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)"))
            }
            pub struct TrickySql3Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql3Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql3Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql3Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql3Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql4() -> TrickySql4Stmt {
                TrickySql4Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)"))
            }
            pub struct TrickySql4Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql4Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql4Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql4Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql4Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql6() -> TrickySql6Stmt {
                TrickySql6Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)"))
            }
            pub struct TrickySql6Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql6Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql6Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql6Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql6Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql7() -> TrickySql7Stmt {
                TrickySql7Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql7Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql7Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql7Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql7Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql7Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql8() -> TrickySql8Stmt {
                TrickySql8Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql8Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql8Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql8Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql8Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql8Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql9() -> TrickySql9Stmt {
                TrickySql9Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql9Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql9Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql9Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql9Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql9Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn tricky_sql10() -> TrickySql10Stmt {
                TrickySql10Stmt(cornucopia_sync::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)"))
            }
            pub struct TrickySql10Stmt(cornucopia_sync::private::Stmt);
            impl TrickySql10Stmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, postgres::Error> {
                    let stmt = self.0.prepare(client)?;
                    client.execute(stmt, &[r#async, r#enum])
                }
            }
            impl<'a, C: GenericClient>
                cornucopia_sync::Params<
                    'a,
                    super::TrickySql10Params,
                    Result<u64, postgres::Error>,
                    C,
                > for TrickySql10Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a mut C,
                    params: &'a super::TrickySql10Params,
                ) -> Result<u64, postgres::Error> {
                    self.bind(client, &params.r#async, &params.r#enum)
                }
            }
            pub fn r#typeof() -> RTypeofStmt {
                RTypeofStmt(cornucopia_sync::private::Stmt::new("SELECT * FROM syntax"))
            }
            pub struct RTypeofStmt(cornucopia_sync::private::Stmt);
            impl RTypeofStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a mut C,
                ) -> TypeofQuery<'a, C, super::Typeof, 0> {
                    TypeofQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::TypeofBorrowed {
                            trick_y: row.get(0),
                            r#async: row.get(1),
                            r#enum: row.get(2),
                        },
                        mapper: |it| <super::Typeof>::from(it),
                    }
                }
            }
        }
        pub mod async_ {
            use cornucopia_async::GenericClient;
            use futures;
            use futures::{StreamExt, TryStreamExt};
            pub struct PublicCloneCompositeQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(
                    &tokio_postgres::Row,
                )
                    -> super::super::super::types::public::CloneCompositeBorrowed,
                mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> PublicCloneCompositeQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::super::super::types::public::CloneCompositeBorrowed) -> R,
                ) -> PublicCloneCompositeQuery<'a, C, R, N> {
                    PublicCloneCompositeQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct Optioni32Query<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> Option<i32>,
                mapper: fn(Option<i32>) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> Optioni32Query<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(Option<i32>) -> R) -> Optioni32Query<'a, C, R, N> {
                    Optioni32Query {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct RowQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::Row,
                mapper: fn(super::Row) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> RowQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(self, mapper: fn(super::Row) -> R) -> RowQuery<'a, C, R, N> {
                    RowQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct RowSpaceQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::RowSpace,
                mapper: fn(super::RowSpace) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> RowSpaceQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::RowSpace) -> R,
                ) -> RowSpaceQuery<'a, C, R, N> {
                    RowSpaceQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub struct TypeofQuery<'a, C: GenericClient, T, const N: usize> {
                client: &'a C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_async::private::Stmt,
                extractor: fn(&tokio_postgres::Row) -> super::TypeofBorrowed,
                mapper: fn(super::TypeofBorrowed) -> T,
            }
            impl<'a, C, T: 'a, const N: usize> TypeofQuery<'a, C, T, N>
            where
                C: GenericClient,
            {
                pub fn map<R>(
                    self,
                    mapper: fn(super::TypeofBorrowed) -> R,
                ) -> TypeofQuery<'a, C, R, N> {
                    TypeofQuery {
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }
                }
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let row = self.client.query_one(stmt, &self.params).await?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }
                pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                    self.iter().await?.try_collect().await
                }
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                    let stmt = self.stmt.prepare(self.client).await?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }
                pub async fn iter(
                    self,
                ) -> Result<
                    impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                    tokio_postgres::Error,
                > {
                    let stmt = self.stmt.prepare(self.client).await?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        .into_stream();
                    Ok(it)
                }
            }
            pub fn select_compact() -> SelectCompactStmt {
                SelectCompactStmt(cornucopia_async::private::Stmt::new("SELECT * FROM clone"))
            }
            pub struct SelectCompactStmt(cornucopia_async::private::Stmt);
            impl SelectCompactStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn select_spaced() -> SelectSpacedStmt {
                SelectSpacedStmt(cornucopia_async::private::Stmt::new(
                    "      SELECT * FROM clone ",
                ))
            }
            pub struct SelectSpacedStmt(cornucopia_async::private::Stmt);
            impl SelectSpacedStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> PublicCloneCompositeQuery<
                    'a,
                    C,
                    super::super::super::types::public::CloneComposite,
                    0,
                > {
                    PublicCloneCompositeQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it.into(),
                    }
                }
            }
            pub fn implicit_compact() -> ImplicitCompactStmt {
                ImplicitCompactStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct ImplicitCompactStmt(cornucopia_async::private::Stmt);
            impl ImplicitCompactStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a Option<T1>,
                    price: &'a Option<f64>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    Optioni32Query {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::ImplicitCompactParams<T1>,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                > for ImplicitCompactStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::ImplicitCompactParams<T1>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn implicit_spaced() -> ImplicitSpacedStmt {
                ImplicitSpacedStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct ImplicitSpacedStmt(cornucopia_async::private::Stmt);
            impl ImplicitSpacedStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a Option<T1>,
                    price: &'a Option<f64>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    Optioni32Query {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| row.get(0),
                        mapper: |it| it,
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::ImplicitSpacedParams<T1>,
                    Optioni32Query<'a, C, Option<i32>, 2>,
                    C,
                > for ImplicitSpacedStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::ImplicitSpacedParams<T1>,
                ) -> Optioni32Query<'a, C, Option<i32>, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn named_compact() -> NamedCompactStmt {
                NamedCompactStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NamedCompactStmt(cornucopia_async::private::Stmt);
            impl NamedCompactStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a T1,
                    price: &'a f64,
                ) -> RowQuery<'a, C, super::Row, 2> {
                    RowQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::Row { id: row.get(0) },
                        mapper: |it| <super::Row>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<'a, super::Params<T1>, RowQuery<'a, C, super::Row, 2>, C>
                for NamedCompactStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::Params<T1>,
                ) -> RowQuery<'a, C, super::Row, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn named_spaced() -> NamedSpacedStmt {
                NamedSpacedStmt(cornucopia_async::private::Stmt::new(
                    "INSERT INTO named (name, price, show) VALUES ($1, $2, false) RETURNING id",
                ))
            }
            pub struct NamedSpacedStmt(cornucopia_async::private::Stmt);
            impl NamedSpacedStmt {
                pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                    &'a mut self,
                    client: &'a C,
                    name: &'a T1,
                    price: &'a f64,
                ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
                    RowSpaceQuery {
                        client,
                        params: [name, price],
                        stmt: &mut self.0,
                        extractor: |row| super::RowSpace { id: row.get(0) },
                        mapper: |it| <super::RowSpace>::from(it),
                    }
                }
            }
            impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
                cornucopia_async::Params<
                    'a,
                    super::ParamsSpace<T1>,
                    RowSpaceQuery<'a, C, super::RowSpace, 2>,
                    C,
                > for NamedSpacedStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::ParamsSpace<T1>,
                ) -> RowSpaceQuery<'a, C, super::RowSpace, 2> {
                    self.bind(client, &params.name, &params.price)
                }
            }
            pub fn tricky_sql() -> TrickySqlStmt {
                TrickySqlStmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a bind_param\', $1, $2)"))
            }
            pub struct TrickySqlStmt(cornucopia_async::private::Stmt);
            impl TrickySqlStmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySqlParams,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySqlStmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySqlParams,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql1() -> TrickySql1Stmt {
                TrickySql1Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a :bind_param', $1, $2)"))
            }
            pub struct TrickySql1Stmt(cornucopia_async::private::Stmt);
            impl TrickySql1Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql1Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql1Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql1Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql2() -> TrickySql2Stmt {
                TrickySql2Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is not a '':bind_param''', $1, $2)"))
            }
            pub struct TrickySql2Stmt(cornucopia_async::private::Stmt);
            impl TrickySql2Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql2Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql2Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql2Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql3() -> TrickySql3Stmt {
                TrickySql3Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum)  VALUES ($$this is not a :bind_param$$, $1, $2)"))
            }
            pub struct TrickySql3Stmt(cornucopia_async::private::Stmt);
            impl TrickySql3Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql3Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql3Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql3Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql4() -> TrickySql4Stmt {
                TrickySql4Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ($tag$this is not a :bind_param$tag$, $1, $2)"))
            }
            pub struct TrickySql4Stmt(cornucopia_async::private::Stmt);
            impl TrickySql4Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql4Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql4Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql4Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql6() -> TrickySql6Stmt {
                TrickySql6Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is not a '':bind_param''', $1, $2)"))
            }
            pub struct TrickySql6Stmt(cornucopia_async::private::Stmt);
            impl TrickySql6Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql6Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql6Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql6Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql7() -> TrickySql7Stmt {
                TrickySql7Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is not a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql7Stmt(cornucopia_async::private::Stmt);
            impl TrickySql7Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql7Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql7Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql7Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql8() -> TrickySql8Stmt {
                TrickySql8Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (e'this is ''not'' a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql8Stmt(cornucopia_async::private::Stmt);
            impl TrickySql8Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql8Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql8Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql8Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql9() -> TrickySql9Stmt {
                TrickySql9Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES (E'this is \'not\' a \':bind_param\'', $1, $2)"))
            }
            pub struct TrickySql9Stmt(cornucopia_async::private::Stmt);
            impl TrickySql9Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql9Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql9Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql9Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn tricky_sql10() -> TrickySql10Stmt {
                TrickySql10Stmt(cornucopia_async::private::Stmt::new("INSERT INTO syntax (\"trick:y\", async, enum) VALUES ('this is just a cast'::text, $1, $2)"))
            }
            pub struct TrickySql10Stmt(cornucopia_async::private::Stmt);
            impl TrickySql10Stmt {
                pub async fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                    r#async: &'a super::super::super::types::public::SyntaxComposite,
                    r#enum: &'a super::super::super::types::public::SyntaxEnum,
                ) -> Result<u64, tokio_postgres::Error> {
                    let stmt = self.0.prepare(client).await?;
                    client.execute(stmt, &[r#async, r#enum]).await
                }
            }
            impl<'a, C: GenericClient + Send + Sync>
                cornucopia_async::Params<
                    'a,
                    super::TrickySql10Params,
                    std::pin::Pin<
                        Box<
                            dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                                + Send
                                + 'a,
                        >,
                    >,
                    C,
                > for TrickySql10Stmt
            {
                fn params(
                    &'a mut self,
                    client: &'a C,
                    params: &'a super::TrickySql10Params,
                ) -> std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                > {
                    Box::pin(self.bind(client, &params.r#async, &params.r#enum))
                }
            }
            pub fn r#typeof() -> RTypeofStmt {
                RTypeofStmt(cornucopia_async::private::Stmt::new("SELECT * FROM syntax"))
            }
            pub struct RTypeofStmt(cornucopia_async::private::Stmt);
            impl RTypeofStmt {
                pub fn bind<'a, C: GenericClient>(
                    &'a mut self,
                    client: &'a C,
                ) -> TypeofQuery<'a, C, super::Typeof, 0> {
                    TypeofQuery {
                        client,
                        params: [],
                        stmt: &mut self.0,
                        extractor: |row| super::TypeofBorrowed {
                            trick_y: row.get(0),
                            r#async: row.get(1),
                            r#enum: row.get(2),
                        },
                        mapper: |it| <super::Typeof>::from(it),
                    }
                }
            }
        }
    }
}
