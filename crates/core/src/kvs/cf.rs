use crate::kvs::Datastore;
use crate::kvs::tasklease::LeaseHandler;
use crate::kvs::{LockType::*, TransactionType::*};
use crate::vs::VersionStamp;
use anyhow::Result;

impl Datastore {
	/// Saves the current timestamp for each database's current versionstamp.
	#[instrument(level = "trace", target = "surrealdb::core::kvs::ds", skip(self, lh))]
	pub(crate) async fn changefeed_versionstamp(
		&self,
		lh: Option<&LeaseHandler>,
		ts: u64,
	) -> Result<Option<VersionStamp>> {
		// Store the latest versionstamp
		let mut vs: Option<VersionStamp> = None;
		// Create a new transaction
		let txn = self.transaction(Write, Optimistic).await?;
		// Fetch all namespaces
		let nss = catch!(txn, txn.all_ns().await);
		// Loop over all namespaces
		for ns in nss.iter() {
			// Get the namespace name
			let ns = &ns.name;
			// Fetch all namespaces
			let dbs = catch!(txn, txn.all_db(ns).await);
			// Loop over all databases
			for db in dbs.iter() {
				// Get the database name
				let db = &db.name;
				// TODO(SUR-341): This is incorrect, it's a [ns,db] to vs pair
				// It's safe for now, as it is unused but either the signature must change
				// to include {(ns, db): (ts, vs)} mapping, or we don't return it
				vs = Some(txn.lock().await.set_timestamp_for_versionstamp(ts, ns, db).await?);
			}
			// Possibly renew the lease
			if let Some(lh) = lh {
				lh.try_maintain_lease().await?;
			}
			// Pause execution
			yield_now!();
		}
		// Commit the changes
		catch!(txn, txn.commit().await);
		// Return the version
		Ok(vs)
	}

	/// Deletes all change feed entries that are older than the timestamp.
	#[instrument(level = "trace", target = "surrealdb::core::kvs::ds", skip(self, lh))]
	pub(crate) async fn changefeed_cleanup(
		&self,
		lh: Option<&LeaseHandler>,
		ts: u64,
	) -> Result<()> {
		// Create a new transaction
		let txn = self.transaction(Write, Optimistic).await?;
		// Perform the garbage collection
		catch!(txn, crate::cf::gc_all_at(lh, &txn, ts).await);
		// Commit the changes
		catch!(txn, txn.commit().await);
		// Everything ok
		Ok(())
	}
}
