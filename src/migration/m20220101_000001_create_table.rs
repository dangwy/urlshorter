use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::TransactionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let db = manager.get_connection();
		let tx = db.begin().await?;

		let db = manager.get_connection();
		let tx = db.begin().await?;
		tx.execute_unprepared(
			r#"CREATE TABLE clients (
			  id UUID NOT NULL primary key,
			  access_key text NOT NULL UNIQUE,
			  secret_key text  NOT NULL,
			  is_active bool NOT NULL default true, 
			  create_at timestamp(6) NOT NULL)"#,
		).await?;
		
		tx.execute_unprepared(
			r#"CREATE UNIQUE INDEX "idx_clients_access_key" ON "clients" USING btree (
				  "access_key"  ASC NULLS LAST)"#
		).await?;
			
		tx.execute_unprepared(
		r#"
            CREATE TABLE "urls" (
              "id" bigserial primary key,
              "domain" text NOT NULL,
              "alias" text NOT NULL,
              "short_url" text NOT NULL,
              "deleted" bool NOT NULL,
              "tags" int8[],
              "created_at" timestamp(6) NOT NULL,
              "expired_at" timestamp(6),
              "original_url" text NOT NULL,
              "description" text,
              "hits" int4 NOT NULL default 0) "#
		).await?;

		tx.execute_unprepared(
		r#"
            CREATE UNIQUE INDEX "idx_domain_alias" ON "urls" USING btree (
              "domain" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST,
              "alias" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST) "#
		).await?;

		tx.execute_unprepared(
		r#" CREATE TABLE "tags" (
			  "id" bigserial primary key,
			  "tag" text NOT NULL,
			  "domain" text NOT NULL,
			  "created_at" timestamp(6) NOT NULL) "#
		).await?;

		tx.execute_unprepared(
		r#" CREATE UNIQUE INDEX "idx_domain_tags" ON "tags" USING btree (
			  "domain" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST,
			  "tag" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST) "#
		).await?;
		tx.commit().await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.get_connection().execute_unprepared(r#" DROP TABLE IF EXISTS clients; "#).await?;
		manager.get_connection().execute_unprepared(r#" DROP TABLE IF EXISTS urls; "#).await?;
		manager.get_connection().execute_unprepared(r#" DROP TABLE IF EXISTS tags; "#).await?;
		
		Ok(())
	}
}
